#[macro_use] extern crate nickel;
extern crate nickel_cookies;
extern crate cookie;
use nickel::{Nickel, HttpRouter, QueryString, Query};
use cookie::Cookie;
use nickel::extensions::Redirect;
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

fn get_sign_in_query (q: &Query) -> (&str, &str) {
    let token = q.get("oauth_token").unwrap();
    let verifier = q.get("oauth_verifier").unwrap();
    return (token, verifier);
}

fn main() {
    let mut server = Nickel::new();
    let pool = model::establish_resourcepool("test.db");
    let mut twitter_client = twitter::new(".config");

    server.get("/", middleware! {|_, response|
        let users = model::get_all_users(&pool);
        let papers = model::get_all_papers(&pool);
        let comments = model::get_all_comments(&pool);
        return response.render("view/index.tmpl", &Giant_Root_Node { users: users, papers: papers, comments: comments });
    });

    server.get("/sign-in", middleware! {|_, response|
        let twitter_sign_in = twitter_client.generate_authorize_url();
        return response.redirect(twitter_sign_in)});

    server.get("/sign-in-callback/", middleware! {|request,response|
        let query = get_sign_in_query(request.query());
    });

    server.listen("127.0.0.1:6767");
}
