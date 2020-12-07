use lang_lib::mecab::convert_sentences;

fn main() {
    let text = include_str!("../../assets/neko.txt.mecab");
    let sentences = convert_sentences(&text);
    let strs: Vec<_> = sentences.into_iter().filter(|sentence|
        sentence.morphemes.iter().any(|morpheme| morpheme.pos == "動詞")
    )
        .map(|sentence|
            sentence.morphemes.into_iter()
                .filter(|morpheme|
                    {
                        morpheme.pos == "動詞"
                    })
                .map(|morpheme| morpheme.base)
                .collect::<Vec<_>>()
        ).collect();

    println!("{:?}", strs)
}