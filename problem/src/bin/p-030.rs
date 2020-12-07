use lang_lib::mecab::convert_sentences;

fn main() {
    let text = include_str!("../../assets/neko.txt.mecab");
    let sentences = convert_sentences(&text);
    println!("{:?}", sentences)
}