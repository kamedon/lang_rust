use lang_lib::mecab::read_file;

fn main() {
    let strs: Vec<_> = read_file("./assets/neko.txt.mecab").into_iter()
        .filter(|sentence|
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