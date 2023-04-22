use std::collections::HashMap;
use lazy_static::lazy_static;
use std::io::prelude::*;
use flate2::read::ZlibDecoder;
use serde_json;

const SCRIPTSDATA: &'static [u8] = include_bytes!("../../tmp/scripts.obj");

lazy_static!{
  pub static ref SCRIPTFILES: HashMap<String, String> = {
    let mut decoder = ZlibDecoder::new(SCRIPTSDATA);
    let mut string_buffer = String::new();
    decoder.read_to_string(&mut string_buffer).expect("Unable to uncompress file data");

    let mut deserialized: HashMap<String, String> = serde_json::from_str(string_buffer.as_str())
      .expect("Unable to deserialze json object");

    let build_file = include_str!("../build_settings.lua");
    deserialized.insert("build_settings.lua".to_string(), build_file.to_string());

    println!("Generated {} files", deserialized.len());

    deserialized
  };
}
