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
//use oauth_client::Token;

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct AppConfig {
    pub consumer_key: String,
    pub consumer_secret: String,
}

pub fn read_appconfig () {
    let mut f = File::open(".config").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    let decoded: AppConfig = toml::decode_str(&s).unwrap();
}


