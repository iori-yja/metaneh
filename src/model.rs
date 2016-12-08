extern crate r2d2;
extern crate r2d2_sqlite;
extern crate rustc_serialize;
extern crate mustache;
use std::thread;
use std::vec;

#[derive(RustcEncodable)]
pub struct User {
    pub user_id: i32,
    pub twitter_id: i64,
    pub screenname: String,
    pub name: String,
}

#[derive(RustcEncodable)]
pub struct Paper {
    pub paper_id: i32,
    pub author_id: i32,
    pub title: String,
    pub abst_url:  String,
    pub comment: String
}

#[derive(RustcEncodable)]
pub struct Comment {
    pub id: i32,
    pub user_id: i32,
    pub comment: String
}

#[derive(RustcEncodable)]
pub struct User_Config {
    pub user_id: i32,
    pub access_key: String,
    pub access_secret: String
}

pub fn establish_resourcepool(db: &str)
    -> r2d2::Pool<r2d2_sqlite::SqliteConnectionManager> {
    let config = r2d2::Config::builder().pool_size(15).build();
    let manager = r2d2_sqlite::SqliteConnectionManager::new(&db);
    return r2d2::Pool::new(config, manager).unwrap();
}

macro_rules! define_get_all {
    ($fname: tt, $table: tt, $typ: ty, $builder: expr) => {
        pub fn $fname (pool: &r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>)
        -> Vec<$typ>{
            let conn = &pool.get().unwrap();
            let query = format!("select * from {}", $table);
            let mut stmt = conn.prepare(&query).unwrap();
            let mut vec = Vec::new();
            let mut p = stmt.query_map(&[], $builder).unwrap();
            for x in p {
                vec.push(x.unwrap());
            }
            return vec;
        }
    }
}

define_get_all!(get_all_users, "users", User,
                |x| {User {user_id: x.get(0), twitter_id: x.get(1), screenname: x.get(2), name: x.get(3)}
                });

define_get_all!(get_all_papers, "papers", Paper,
                |x| {Paper {paper_id:x.get(0), author_id: x.get(1), title: x.get(2), abst_url: x.get(3), comment: x.get(4)}
                });

define_get_all!(get_all_comments, "comments", Comment,
                |x| {Comment {id:x.get(0), user_id: x.get(1), comment: x.get(2)}
                });


impl User_Config {
    fn get_user_config (pool: &r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>, user_id: i32)
        -> (String, String) {
        let conn = &pool.get().unwrap();
        let query = "select access_key, access_secret from user_config where user_id = $1";
        let ret = conn.query_row(query, &[&user_id], |row| (row.get(0), row.get(1))).unwrap();
        return ret;
    }
    fn set_user_config (pool: &r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>, user_config: User_Config) {
        let conn = &pool.get().unwrap();
        let query = "insert into user_config(user_id, access_key, access_secret) values($1, $2, $3)";
        conn.execute(query, &[&user_config.user_id, &user_config.access_key, &user_config.access_secret]);
    }
}

impl User {
    fn push(&self, pool: r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>)
        -> bool {
        let conn = pool.get().unwrap();
        match conn.execute("INSERT INTO users values ($1, $2, $3, $4)",
                                &[&self.user_id, &self.twitter_id,
                                  &self.screenname, &self.name]
                          ) {
            Ok(_) => return true,
            _ => return false
        };
    }
}

