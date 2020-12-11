use lang_lib::mecab::convert_sentences;
use std::collections::HashMap;

fn main() {
    let text = include_str!("../../assets/neko.txt.mecab");
    let sentences = convert_sentences(&text);
    let mut hash_map: HashMap<&str, u32> = HashMap::new();
    sentences.iter()
        .filter(|sentence|
            sentence.morphemes.iter().any(|morpheme| morpheme.surface == "猫")
        )
        .for_each(|sentence| {
            sentence.morphemes.iter().for_each(|morpheme| {
                let key = &morpheme.surface;
                *hash_map.entry(key).or_default() += 1;
            })
        });

    let mut vec: Vec<_> = hash_map.iter().collect();
    vec.sort_by(|a, b| b.1.cmp(a.1));


    println!("{:?}", vec)
}