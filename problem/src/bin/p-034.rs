use lang_lib::mecab::{convert_sentences, Morpheme};
use itertools::Itertools;

fn main() {
    let text = include_str!("../../assets/neko.txt.mecab");
    let sentences = convert_sentences(&text);
    let mut list: Vec<Vec<Morpheme>> = vec![vec![]];
    for sentence in sentences {
        for morpheme in sentence.morphemes {
            if morpheme.pos == "名詞" {
                list.last_mut().unwrap().push(morpheme)
            } else {
                if !list.last().unwrap().is_empty() {
                    list.push(vec![])
                }
            }
        }
    }

    let ans = list.iter().map(|morphemes| morphemes.iter().map(|morpheme| morpheme.base).join("")).collect_vec();
    println!("{:?}", ans)
}