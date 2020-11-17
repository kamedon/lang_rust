use lang_lib::file::reader;
use std::io::BufRead;

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


fn main() {
    let reader_file_gz = reader("./assets/jawiki-country.json.gz");
    for json in reader_file_gz.lines() {
        let json = json.unwrap();
        let v: JaWikiCountry = serde_json::from_str(&*json).unwrap();
        println!("{:?}", v);
    }
}