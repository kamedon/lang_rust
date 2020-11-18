use lang_lib::file::reader;
use std::io::BufRead;
use regex::{Regex, Captures, Error};
use std::fs::read_to_string;

fn main() {
    let reader_file_gz = reader("./assets/jawiki-country.json.gz");
    let re = Regex::new(r"[File|ファイル]:(.*?)\|").unwrap();
    for json in reader_file_gz.lines() {
        let json = json.unwrap();
        for line in json.split("\\n").into_iter() {
            for cap in re.captures_iter(line) {
                println!("{}", &cap[1]);
            }
        }
    }
}

