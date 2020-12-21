use lang_lib::cabocha::convert_sentences;

fn main() {
    let sentences = convert_sentences(include_str!("../../assets/ai.ja.txt.parsed"));
    println!("{:?}", sentences)
}