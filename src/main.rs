#[macro_use] extern crate nickel;
use std::collections::HashMap;
use nickel::{Nickel, HttpRouter};
extern crate r2d2;
extern crate r2d2_sqlite;
extern crate rustc_serialize;
extern crate mustache;

mod model;

/* Unfortunately, we are force to use majestic bogos structure. */
#[derive(RustcEncodable)]
pub struct Giant_Root_Node {
    users: Vec<model::User>,
    papers: Vec<model::Paper>,
//    comments: Vec<model::Comment>
}


fn main() {
    let mut server = Nickel::new();
    let pool = model::establish_resourcepool("test.db");

    server.get("/", middleware! {|_, response|
        let users = model::get_all_users(&pool);
        let papers = model::get_all_papers(&pool);
        return response.render("view/index.tmpl", &Giant_Root_Node { users: users, papers: papers });
    });

    server.listen("127.0.0.1:6767");
}
