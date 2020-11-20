use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    for result in BufReader::new(File::open("./assets/neko.txt").unwrap()).lines() {
        let l = result.unwrap();
        println!("{}", l);
    }

}