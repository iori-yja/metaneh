#[macro_use] extern crate nickel;
use std::collections::HashMap;
use nickel::{Nickel, HttpRouter, QueryString, Query};
use nickel::extensions::Redirect;
extern crate r2d2;
extern crate r2d2_sqlite;
extern crate rustc_serialize;
extern crate mustache;

mod model;
mod twitter;

/* Unfortunately, we are force to use majestic bogos structure. */
#[derive(RustcEncodable)]
pub struct Giant_Root_Node {
    users: Vec<model::User>,
    papers: Vec<model::Paper>,
    comments: Vec<model::Comment>
}

fn get_sign_in_param (q: &Query) -> (String, String) {
    let token = q.get("oauth_token").unwrap();
    let verifier = q.get("oauth_verifier").unwrap();
    return (token.to_string(), verifier.to_string());
}

fn concat_tuple ((s, t): (String, String)) -> String {
    return s + &t;
}

fn main() {
    let mut server = Nickel::new();
    let pool = model::establish_resourcepool("test.db");

    server.get("/", middleware! {|_, response|
        let users = model::get_all_users(&pool);
        let papers = model::get_all_papers(&pool);
        let comments = model::get_all_comments(&pool);
        return response.render("view/index.tmpl", &Giant_Root_Node { users: users, papers: papers, comments: comments });
    });

    server.get("/sign-in", middleware! {|_, response|
        let twitter_sign_in = twitter::start_sign_in();
        return response.redirect(twitter_sign_in)});

    server.get("/sign-in-callback/", middleware! {|request,response|
        format!("test{}", concat_tuple(get_sign_in_param(request.query())))
    });

    server.listen("127.0.0.1:6767");
}
