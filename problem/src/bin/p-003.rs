fn main() {
    let str = "Now I need a drink, alcoholic of course, after the heavy lectures involving quantum mechanics.";
    let mut vec: Vec<_> = str.split_whitespace().into_iter().map(|x| (x.len())).collect();
    for item in vec {
        println!("{}", item)
    }
}