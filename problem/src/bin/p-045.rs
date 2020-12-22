use lang_lib::cabocha::{convert_sentences, Morph, Chunk};
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write, BufReader, BufWriter};

fn f<'a>(c: &'a Chunk) -> Vec<&'a str> {
    c.morphs.iter().map(|m| m.surface).collect::<Vec<_>>()
}

fn main() {
    let sentences = convert_sentences(include_str!("../../assets/ai.ja.txt.parsed"));
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("output/045.txt")
        .unwrap();
    let mut writer = BufWriter::new(file);
    for sentence in sentences {
        let chunks = sentence.chunks;
        for chunk in chunks.iter() {
            let m = chunk.morphs.iter().find(|m| m.pos == "動詞");
            match m {
                None => {}
                Some(m) => {
                    let words = chunks.iter()
                        .filter(|&c| c.srcs == chunk.dst as i32)
                        .map(|c| f(c))
                        .sorted()
                        .flatten().join(" ");

                    let line = format!("{}\t{}\n", m.base, words);
                    println!("{}", line);

                    writer.write(line.as_bytes()).unwrap();
                }
            }
        }
    }
}