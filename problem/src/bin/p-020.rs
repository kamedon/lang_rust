use flate2::read;
use std::error::Error;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::Path;
use serde::de::Unexpected::Str;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[derive(Debug, Serialize, Deserialize)]
struct Country {
    title: String
}

#[derive(Debug, Serialize, Deserialize)]
struct JaWikiCountry {
    title: String
}

pub fn reader(filename: &str) -> Box<dyn BufRead> {
    let path = Path::new(filename);
    let file = File::open(&path).unwrap();
    if path.extension() == Some(OsStr::new("gz")) {
        Box::new(BufReader::with_capacity(
            128 * 1024,
            read::GzDecoder::new(file),
        ))
    } else {
        Box::new(BufReader::with_capacity(128 * 1024, file))
    }
}


fn main() {
    let reader_file_gz = reader("./assets/jawiki-country.json.gz");
    for json in reader_file_gz.lines() {
        let json = json.unwrap();
        let v: JaWikiCountry = serde_json::from_str(&*json).unwrap();
        println!("{:?}", v);
    }
}