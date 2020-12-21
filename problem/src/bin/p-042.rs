use lang_lib::cabocha::convert_sentences;
use itertools::Itertools;

fn main() {
    let sentences = convert_sentences(include_str!("../../assets/ai.ja.txt.parsed"));
    for sentence in sentences {
        let chunks = &sentence.chunks;
        for chunk in chunks {
            if chunk.srcs == -1 {
                let text = chunk.morphs.iter().map(|morph| morph.surface).join("");
                println!("{:?}", text);
                continue;
            }
            let srcs = chunk.srcs as usize;

            let text = chunk.morphs.iter().map(|morph| morph.surface).join("");
            let srcsText = chunks.get(srcs).unwrap().morphs.iter().map(|morph| morph.surface).join("");
            println!("{}: {}", text, srcsText);
        }
    }
}