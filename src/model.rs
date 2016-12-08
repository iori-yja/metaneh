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
    pub name: String
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
            let query = "select * from users";
            let mut stmt = conn.prepare(query).unwrap();
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
                |x| { User {user_id: x.get(0), twitter_id: x.get(1), screenname: x.get(2), name: x.get(3)}
                });


pub fn get_all_papers (pool: &r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>)
    -> Vec<Paper> {
    let conn = &pool.get().unwrap();
    let query = "select * from papers";
    let mut stmt = conn.prepare(query).unwrap();
    let mut vec = Vec::new();
    let mut p = stmt.query_map(&[], |x| {
        Paper {
            paper_id:   x.get(0),
            author_id:  x.get(1),
            title:      x.get(2),
            abst_url:   x.get(3),
            comment:    x.get(4),
        }
    }).unwrap();

    for x in p {
        vec.push(x.unwrap());
    }
    return vec;
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

