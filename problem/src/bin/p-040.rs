use lang_lib::mecab::readFile;

fn main() {
    let sentences = readFile("./assets/neko.txt.mecab");
    println!("{:?}", sentences)
}