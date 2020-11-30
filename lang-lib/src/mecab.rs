use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Morpheme {
    //表層系
    surface: String,
    //基本形
    base: String,
    //品詞
    pos: String,
    //品詞細分類1
    pos1: String,
}

#[derive(Debug)]
pub struct Sentence {
    morphemes: Vec<Morpheme>
}

pub fn convert_morpheme(str: &str) -> Morpheme {
    let mut split_line = str.split("\t");
    let surface = split_line.next().unwrap();
    let other: Vec<_> = split_line.next().unwrap().split(",").collect();
    Morpheme {
        surface: String::from(surface),
        base: String::from(other[0]),
        pos: String::from(other[1]),
        pos1: String::from(other[2]),
    }
}

pub fn readFile(path: &str) -> Vec<Sentence> {
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
