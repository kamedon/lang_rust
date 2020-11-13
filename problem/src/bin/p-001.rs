use std::iter::FromIterator;

fn main() {
    let str = "stressed";
    let revStr: Vec<_> = str.chars().into_iter().rev().collect();
    println!("{}", String::from_iter(revStr))
}