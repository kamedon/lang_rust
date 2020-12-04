use lang_lib::mecab::read_file;
use std::collections::HashMap;

fn main() {
    let mut hash_map: HashMap<&str, u32> = HashMap::new();
    let sentences = read_file("./assets/neko.txt.mecab");
    sentences.iter().for_each(|sentence| {
        sentence.morphemes.iter().for_each(|morpheme| {
            let key = &morpheme.surface;
            *hash_map.entry(key).or_default() += 1;
        })
    });

    let mut vec: Vec<_> = hash_map.iter().collect();
    vec.sort_by(|a, b| b.1.cmp(a.1));
    println!("{:?}", vec);
}
