#[macro_use] extern crate nickel;
extern crate nickel_cookies;
extern crate cookie;
extern crate rustc_serialize;
extern crate mustache;
use nickel::{Nickel, HttpRouter, QueryString, Query};
use cookie::Cookie;
use nickel::extensions::Redirect;
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

fn get_sign_in_query (q: &Query) -> (&str, &str) {
    let token = q.get("oauth_token").unwrap();
    let verifier = q.get("oauth_verifier").unwrap();
    return (token, verifier);
}

fn main() {
    let mut server = Nickel::new();
    let pool = model::establish_resourcepool("test.db");
    let twitter_client = Arc::new(Mutex::new(twitter::new(".config")));

    server.get("/", middleware! {|_, response|
        let users = model::get_all_users(&pool);
        let papers = model::get_all_papers(&pool);
        let comments = model::get_all_comments(&pool);
        return response.render("view/index.tmpl", &Giant_Root_Node { users: users, papers: papers, comments: comments });
    });

    server.get("/sign-in/:state", middleware! {|request, response|
        print!("[sign-in]: ");
        let mut twitter_sign_in = twitter_client.lock().unwrap();
        match request.param("state") {
            Some("new")      => return response.redirect(twitter_sign_in.generate_authorize_url().to_string()),
            Some(_) => {
                print!("try to ");
                let res = twitter_sign_in.access_token(request.query().get("oauth_verifier").unwrap().to_string());
                match res {
                    Some(user) => {
                            println!("render: {}", user);
                            return response.render("view/logintest.tmpl", &TempResponse{username: user })
                        },
                    None       => {
                            println!("redirect");
                            return response.redirect("/sign-in/new")
                        }
                    }
                },
            None             => return response.redirect("/sign-in/new"),
        }
    });

    print!("running server..");
    server.listen("127.0.0.1:6767");
}
