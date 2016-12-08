extern crate twitter_api;
extern crate rustc_serialize as rustc_serialize;
extern crate oauth_client;
extern crate toml;

use std::convert::AsRef;
use std::io::prelude::*;
use std::env;
use std::fs::{File, OpenOptions};
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use rustc_serialize::Decodable;
use rustc_serialize::json::{self, Json};

#[derive(Debug, RustcEncodable, RustcDecodable)]
struct AppConfig {
    consumer_key: String,
    consumer_secret: String,
}

pub struct Twitter_Authorizer<'a> {
    consumer: oauth_client::Token<'a>,
    request_token_pool: HashMap<i64, oauth_client::Token<'a>>,
}

pub fn new<'a> (config: &str) -> Twitter_Authorizer<'a> {
    let mut f = File::open(config).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    let decoded: AppConfig = toml::decode_str(&s).unwrap();
    Twitter_Authorizer {
        consumer: oauth_client::Token::new(decoded.consumer_key, decoded.consumer_secret),
        request_token_pool: HashMap::new()
    }
}

impl<'a> Twitter_Authorizer<'a> {
    fn obtain_access_token (&self, oauth_verifier: &str) -> String {
        let request = &self.request_token_pool.get(&1).unwrap();
        let access = twitter_api::get_access_token(&self.consumer, request, &oauth_verifier).unwrap();
        let ts = twitter_api::get_last_tweets(&self.consumer, &access).unwrap();
        for t in ts {
            println!("{} - {}", t.created_at, t.text);
        }
    }
    fn generate_authorize_url (&self) -> String {
        let request = twitter_api::get_request_token(&self.consumer).unwrap();
        &self.insert(1, request);
        return twitter_api::get_authorize_url(&request);
    }
}

