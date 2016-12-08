extern crate twitter_api;
extern crate rustc_serialize as rustc_serialize;
extern crate oauth_client;
extern crate toml;

use std::convert::AsRef;
use std::io::prelude::*;
use std::env;
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::path::PathBuf;
use rustc_serialize::Decodable;
use rustc_serialize::json::{self, Json};

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct AppConfig {
    pub consumer_key: String,
    pub consumer_secret: String,
}

fn read_appconfig () -> AppConfig {
    let mut f = File::open(".config").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    let decoded: AppConfig = toml::decode_str(&s).unwrap();
    return decoded;
}

fn generate_authorize_url (ac: AppConfig) -> String {
    let consumer = oauth_client::Token::new(ac.consumer_key, ac.consumer_secret);
    let request = twitter_api::get_request_token(&consumer).unwrap();
    return twitter_api::get_authorize_url(&request);
}

pub fn start_sign_in () -> String {
    return generate_authorize_url(read_appconfig());
}
