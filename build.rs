use std::collections::HashMap;
use std::{fs, io::{self, prelude::*}};
use flate2::Compression;
use flate2::write::ZlibEncoder;
use serde_json;

fn main() {
  let entries = fs::read_dir("src/scripts")
    .expect("src/scripts is not found")
    .map(|res| res.map(|e| e.path()))
    .collect::<Result<Vec<_>, io::Error>>()
    .expect("Unable to iterate over scripts folder");

  let mut file_datas: HashMap<String, String> = HashMap::new();

  for entry in entries {
    if entry.is_dir() { continue; }

    let file_name = entry.as_path().to_string_lossy().to_string()
      .replace("\\", "/")
      .replace("src/scripts/", "");
    let file_contents = fs::read_to_string(entry.as_path())
      .expect("Unable to read file contents");

    file_datas.insert(file_name, file_contents);
  }

  let json_string = serde_json::to_string(&file_datas)
    .expect("Failed to serialize HashMap");

  let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
  encoder.write_all(&json_string.as_bytes())
    .expect("Unable to write json string to compression buffer");

  let compressed_bytes = encoder.finish().expect("Unable to encode file data");
  
  fs::write("tmp/buildobj.obj", compressed_bytes)
    .expect("Unable to write to tmp/buildobj.obj")
}