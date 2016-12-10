extern crate rustc_serialize as rustc_serialize;
extern crate toml;
extern crate egg_mode;
extern crate bmemcached;

use std::convert::AsRef;
use std::clone::Clone;
use std::io::prelude::*;
use std::env;
use std::fs::{File, OpenOptions};
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use rustc_serialize::Decodable;
use rustc_serialize::json::{self, Json};
use self::bmemcached::errors::BMemcachedError;
use std::ops::Deref;

#[derive(Debug, RustcEncodable, RustcDecodable)]
struct AppConfig {
    consumer_key: String,
    consumer_secret: String,
}

pub struct Twitter_Authorizer<'a> {
    consumer: egg_mode::Token<'a>,
    request_token_pool: bmemcached::MemcachedClient
}

pub fn new<'a> (config: &str) -> Twitter_Authorizer<'a> {
    let mut f = File::open(config).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    let decoded: AppConfig = toml::decode_str(&s).unwrap();
    Twitter_Authorizer {
        consumer: egg_mode::Token::new(decoded.consumer_key, decoded.consumer_secret),
        request_token_pool: bmemcached::MemcachedClient::new(vec!["localhost:11211"], 6).unwrap()
    }
}

impl<'a> Twitter_Authorizer<'a> {
    pub fn access_token (&self, oauth_verifier: String) -> Option<String> {
        let req_key : String = self.request_token_pool.get("key").unwrap();
        let req_sec : String = self.request_token_pool.get("sec").unwrap();

        println!("got: {} - {}", req_key, req_sec);
        let request = egg_mode::Token::new(req_key, req_sec);

        match egg_mode::access_token(&self.consumer, &request, oauth_verifier) {
            Ok((token, id, name)) => {
                println!("id: {}, name: {}", id, name);
                return Some(name);
            }
            Err(e) => {
                println!("{}", e);
                return None;
            }
        }
    }

    pub fn generate_authorize_url (&self) -> String {
        let mut request = egg_mode::request_token(&self.consumer, "http://localhost:6767/sign-in/callback").unwrap();
        let url = egg_mode::authenticate_url(&request);
        let key = &request.key.into_owned();
        let sec = &request.secret.into_owned();
        println!("{}: {}", key, sec);
        &self.request_token_pool.add("key", key, 60);
        &self.request_token_pool.add("sec", sec, 60);
        return url;
    }
}

