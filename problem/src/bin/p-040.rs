use regex::{Regex, Captures};

#[derive(Debug)]
struct Morph<'a> {
    pub surface: &'a str,
    pub base: &'a str,
    pub pos: &'a str,
    pub pos1: &'a str,
}

#[derive(Debug)]
struct Sentence<'a> {
    pub morphs: Vec<Morph<'a>>
}

impl<'b> Sentence<'b> {
    pub fn new<'a>() -> Sentence<'a> {
        Sentence { morphs: vec![] }
    }
}

impl<'b> Morph<'b> {
    pub fn new(text: &str) -> Morph {
        println!("{}", text);
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

fn main() {
    let text = include_str!("../../assets/ai.ja.txt.parsed");

    let mut sentences = vec![Sentence::new()];
    let re = Regex::new(r"\* (\S+) (\S+) (\S+) (\S+)$").unwrap();

    for line in text.lines() {
        match line {
            "EOS" => { sentences.push(Sentence::new()) }
            _ => {
                match re.captures(line) {
                    None => {
                        sentences.last_mut().unwrap().morphs.push(Morph::new(line))
                    }
                    Some(_) => {}
                }
            }
        }
    }

    println!("{:?}", sentences)
}