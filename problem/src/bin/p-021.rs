use lang_lib::file::reader;
use std::io::BufRead;
use regex::{Regex};

fn main() {
    let reader_file_gz = reader("./assets/jawiki-country.json.gz");
    let re = Regex::new(r"^\[\[Category:.*\]\]$").unwrap();
    for json in reader_file_gz.lines() {
        let json = json.unwrap();
        for line in json.split("\\n").into_iter() {
            if re.is_match(line) {
                println!("{}", line)
            }
        }
    }
}