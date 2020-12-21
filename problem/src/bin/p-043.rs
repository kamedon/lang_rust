use lang_lib::cabocha::convert_sentences;
use itertools::Itertools;

fn main() {
    let sentences = convert_sentences(include_str!("../../assets/ai.ja.txt.parsed"));
    for sentence in sentences {
        let chunks = &sentence.chunks;
        for chunk in chunks {

            //かかりつけなし
            if chunk.srcs == -1 {
                continue;
            }

            let srcs = chunk.srcs as usize;
            let srcs_chunk = chunks.get(srcs).unwrap();

            let is_noun = chunk.morphs.iter().any(|morph| morph.pos == "名詞");
            let is_verb = srcs_chunk.morphs.iter().any(|morph| morph.pos == "動詞");
            if is_noun && is_verb {
                let text = chunk.morphs.iter().map(|morph| morph.surface).join("");
                let srcsText = srcs_chunk.morphs.iter().map(|morph| morph.surface).join("");
                println!("{}\t {}", text, srcsText);
            }
        }
    }
}