use lang_lib::file::reader;
use std::io::BufRead;
use regex::{Regex, Captures, Error};
use std::fs::read_to_string;

fn main() {
    let reader_file_gz = reader("./assets/jawiki-country.json.gz");
    for json in reader_file_gz.lines() {
        let json = json.unwrap();
        for line in json.split("\\n").into_iter() {
            let section = regex_section(0, line);
            if section != None {
                println!("{:?}", section.unwrap())
            }
        }
    }
}

fn regex_section(level: u32, str: &str) -> Option<(u32, &str)> {
    let re = Regex::new(r"^=(.*)=$").unwrap();
    let caps = re.captures(str);
    return match caps {
        None => {
            if level == 0 {
                None
            } else {
                Some((level, str))
            }
        }
        Some(caps) => {
            let s = caps.get(1).unwrap().as_str();
            return regex_section(level + 1, s.trim());
        }
    };
}