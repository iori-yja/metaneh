#[macro_use] extern crate nickel;
use std::collections::HashMap;
use nickel::{Nickel, HttpRouter};
extern crate r2d2;
extern crate r2d2_sqlite;
extern crate rustc_serialize;
extern crate mustache;

mod model;

fn main() {
    let mut server = Nickel::new();
    let pool = model::establish_resourcepool("test.db");

    server.get("/", middleware! {|_, response|
        let mut data = HashMap::new();
        let users = model::get_all_users(&pool);
        data.insert("users", users);
        return response.render("view/index.tmpl", &data);
    });

    server.listen("127.0.0.1:6767");
}
