#[macro_use] extern crate nickel;
extern crate nickel_cookies;
extern crate cookie;
extern crate rustc_serialize;
extern crate mustache;
extern crate bmemcached;
use nickel::{Nickel, HttpRouter, QueryString, Query};
use nickel::status::StatusCode;
use cookie::Cookie;
use nickel::extensions::Redirect;
use std::panic;
use std::sync::{Arc, Mutex};

mod model;
mod twitter;

/* Unfortunately, we are force to use majestic bogos structure. */
#[derive(RustcEncodable)]
pub struct Giant_Root_Node {
    users: Vec<model::User>,
    papers: Vec<model::Paper>,
    comments: Vec<model::Comment>
}

#[derive(RustcEncodable)]
pub struct TempResponse {
    username: String,
}

fn main() {
    let mut server = Nickel::new();
    let pool = model::establish_resourcepool("test.db");
    let twitter_client = Arc::new(twitter::new(".config"));

    server.get("/", middleware! {|_, response|
        let users = model::get_all_users(&pool);
        let papers = model::get_all_papers(&pool);
        let comments = model::get_all_comments(&pool);
        return response.render("view/index.tmpl", &Giant_Root_Node { users: users, papers: papers, comments: comments });
    });

    server.get("/sign-in/:state", middleware! {|request, response|
        match request.param("state") {
            Some("new")      => return response.redirect(twitter_client.generate_authorize_url().to_string()),
            Some("callback") => {
                let verifier = request.query().get("oauth_verifier");
                if verifier.is_none() { return response.error(StatusCode::BadRequest, "") };
                let res = twitter_client.access_token(verifier.unwrap().to_string());
                match res {
                    Some(user) => return response.render("view/logintest.tmpl", &TempResponse{username: user}),
                    _  => return response.redirect("/sign-in/new")
                    }
                },
            _                  => return response.redirect("/sign-in/new"),
        }
    });

    server.listen("127.0.0.1:6767");
}
