use lang_lib::file::reader;
use std::io::BufRead;
use regex::{Regex, Captures, Error};
use std::fs::read_to_string;

fn main() {
    let reader_file_gz = reader("./assets/jawiki-country.json.gz");
    let basic_info_re = Regex::new(r"\{\{基礎情報(.*)\}\}").unwrap();
    let field_re = Regex::new(r"\|(.+?)=(.+?)\\n").unwrap();
    let remove_string_re = Regex::new(r"'{2,5}").unwrap();
    for json in reader_file_gz.lines().take(1) {
        let json = json.unwrap();
        let str = remove_string_re.replace_all(json.as_str(), "");
        for cap in basic_info_re.captures_iter(str.as_ref()) {
            for cap2 in field_re.captures_iter(&cap[1]) {
                println!("{} = {}", &cap2[1].trim(), &cap2[2]);
            }
        }
    }
}
