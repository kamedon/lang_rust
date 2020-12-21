use regex::{Regex, Captures};

#[derive(Debug)]
pub struct Morph<'a> {
    pub surface: &'a str,
    pub base: &'a str,
    pub pos: &'a str,
    pub pos1: &'a str,
}

#[derive(Debug)]
pub struct Chunk<'a> {
    pub text: &'a str,
    // 係り先文節インデックス番号
    pub dst: u32,
    // 係り元文節インデックス番号のリスト
    pub srcs: i32,
    pub morphs: Vec<Morph<'a>>,
}

#[derive(Debug)]
pub struct Sentence<'a> {
    pub chunks: Vec<Chunk<'a>>
}


impl<'b> Sentence<'b> {
    pub fn new<'a>() -> Sentence<'a> {
        Sentence {
            chunks: vec![]
        }
    }
}

impl<'b> Chunk<'b> {
    pub fn new(text: &str, dst: u32, srcs: i32) -> Chunk {
        Chunk {
            text,
            dst,
            srcs,
            morphs: vec![],
        }
    }
}


impl<'b> Morph<'b> {
    pub fn new(text: &str) -> Morph {
        let mut split_text = text.split("\t");
        let surface = split_text.next().unwrap();
        let other = split_text.next().unwrap();
        let mut other = other.split(",");
        let pos = other.next().unwrap();
        let pos1 = other.next().unwrap();
        let base = other.skip(4).next().unwrap();
        Morph {
            surface,
            base,
            pos,
            pos1,
        }
    }
}

pub fn convert_sentences(text: &str) -> Vec<Sentence> {
    let mut sentences = vec![Sentence::new()];
    let re = Regex::new(r"\* (\S+) (\S+)D (\S+) (\S+)$").unwrap();

    for line in text.lines() {
        match line {
            "EOS" => { sentences.push(Sentence::new()) }
            _ => {
                match re.captures(line) {
                    None => {
                        sentences.last_mut().unwrap().chunks.last_mut().unwrap().morphs.push(Morph::new(line))
                    }
                    Some(caps) => {
                        let dst: u32 = *&caps[1].parse().unwrap();
                        let srcs: i32 = *&caps[2].parse().unwrap();
                        sentences.last_mut().unwrap().chunks.push(Chunk::new(line, dst, srcs))
                    }
                }
            }
        }
    }
    sentences
}
