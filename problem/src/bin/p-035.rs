use lang_lib::mecab::read_file;
use std::collections::HashMap;

fn main() {
    let mut hash_map: HashMap<String, u32> = HashMap::new();
    read_file("./assets/neko.txt.mecab").into_iter()
        .for_each(|sentence| {
            sentence.morphemes.iter().for_each(|morpheme| {
                let key = morpheme.surface.clone();
                let current = match hash_map.get(&*key) {
                    None => { 1 }
                    Some(v) => { *v + 1 }
                };
                hash_map.insert(key, current);
            })
        });

    let mut vec: Vec<(&String, &u32)> = hash_map.iter().collect();
    vec.sort_by(|a, b| b.1.cmp(a.1));
    println!("{:?}", vec)
}