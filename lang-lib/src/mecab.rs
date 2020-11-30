use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Morpheme {
    //表層系
    pub surface: String,
    //基本形
    pub base: String,
    //品詞
    pub pos: String,
    //品詞細分類1
    pub pos1: String,
}

#[derive(Debug)]
pub struct Sentence {
    pub morphemes: Vec<Morpheme>
}

pub fn convert_morpheme(str: &str) -> Morpheme {
    let mut split_line = str.split("\t");
    let surface = split_line.next().unwrap();
    let other: Vec<_> = split_line.next().unwrap().split(",").collect();
    Morpheme {
        surface: String::from(surface),
        pos: String::from(other[0]),
        pos1: String::from(other[1]),
        base: String::from(other[6]),
    }
}

pub fn read_file(path: &str) -> Vec<Sentence> {
    let mut sentences: Vec<Sentence> = vec![Sentence { morphemes: vec![] }];

    for line in BufReader::new(File::open(path).unwrap()).lines() {
        let line = line.unwrap();
        if line == "EOS" {
            sentences.push(Sentence {
                morphemes: vec![]
            });
            continue;
        }
        let morpheme = convert_morpheme(&line);
        sentences.last_mut().unwrap().morphemes.push(morpheme)
    }
    sentences
}
