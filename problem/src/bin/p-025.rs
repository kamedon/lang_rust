use lang_lib::file::reader;
use std::io::BufRead;
use regex::{Regex, Captures, Error};
use std::fs::read_to_string;

fn main() {
    let reader_file_gz = reader("./assets/jawiki-country.json.gz");
    let basic_info_re = Regex::new(r"\{\{基礎情報(.*)\}\}").unwrap();
    let field_re = Regex::new(r"\|(.+?)=(.+?)\\n").unwrap();
    // let field_re = Regex::new(r"\|(.*)=(.*)\\n").unwrap();
    for json in reader_file_gz.lines().take(3) {
        let json = json.unwrap();
        for cap in basic_info_re.captures_iter(json.as_str()) {
            // println!("{}", &cap[1]);
            for cap2 in field_re.captures_iter(&cap[1]) {
                println!("{} = {}", &cap2[1].trim(), &cap2[2].trim());
            }
        }
    }

}

