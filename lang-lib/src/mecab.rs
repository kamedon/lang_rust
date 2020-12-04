use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Morpheme<'a> {
    //表層系
    pub surface: &'a str,
    //基本形
    pub base: &'a str,
    //品詞
    pub pos: &'a str,
    //品詞細分類1
    pub pos1: &'a str,
}

#[derive(Debug)]
pub struct Sentence<'a> {
    pub morphemes: Vec<Morpheme<'a>>
}

pub fn to_morpheme(line: &str) -> Morpheme {
    let mut split_line = line.split("\t");
    let surface = split_line.next().unwrap();
    let mut other = split_line.next().unwrap().split(",");
    let pos = other.next().unwrap();
    let pos1 = other.next().unwrap();
    let base = other.skip(4).next().unwrap();


    Morpheme {
        surface,
        pos,
        pos1,
        base,
    }
}

pub fn read_file(path: &str) -> Vec<Sentence> {
    // let file = BufReader::new(File::open(path).unwrap());
    let file = include_str!("../../problem/assets/neko.txt.mecab");
    let mut sentences: Vec<Sentence> = vec![Sentence { morphemes: vec![] }];
    let lines = file.lines();
    for line in lines {
        if line == "EOS" {
            sentences.push(Sentence {
                morphemes: vec![]
            });
            continue;
        }
        let morpheme = to_morpheme(&line);
        sentences.last_mut().unwrap().morphemes.push(morpheme)
    }
    sentences
}
