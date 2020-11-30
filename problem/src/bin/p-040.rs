use lang_lib::mecab::read_file;

fn main() {
    let sentences = read_file("./assets/neko.txt.mecab");
    println!("{:?}", sentences)
}