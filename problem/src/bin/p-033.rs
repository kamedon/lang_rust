use lang_lib::mecab::convert_sentences;
use itertools::Itertools;

fn main() {
    let text = include_str!("../../assets/neko.txt.mecab");
    let sentences = convert_sentences(&text);
    let ans = sentences.iter().map(|sentence| {
        sentence.morphemes.windows(3).filter(|&m| {
            m[0].pos == "名詞" && m[1].base == "の" && m[2].pos == "名詞"
        }).map(|m| (m[0].base, m[1].base, m[2].base)).collect_vec()
    }).filter(|m| !m.is_empty())
        .collect_vec();
    println!("{:?}", ans)
}