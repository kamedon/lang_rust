use lang_lib::cabocha::{convert_sentences, Morph, Chunk};
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write, BufReader, BufWriter};

//
fn f<'a>(c: &'a Chunk) -> Vec<&'a str> {
    let w = c.morphs.iter().filter(|m| m.pos == "助詞").map(|m| m.surface).sorted().collect::<Vec<_>>();
    w
}

fn f2(c: &Chunk) -> String {
    let w2 = c.morphs.iter().map(|m| m.base).join("");
    w2
}

fn main() {
    let sentences = convert_sentences(include_str!("../../assets/ai.ja.txt.parsed"));
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
                        .flatten()
                        .join(" ");

                    let word2 = chunks.iter()
                        .filter(|&c| c.srcs == chunk.dst as i32)
                        .map(|c| f2(c))
                        .sorted()
                        .join(" ");

                    let line = format!("{}\t{} {}\n", m.base, words, word2);
                    println!("{}", line);
                }
            }
        }
    }
}