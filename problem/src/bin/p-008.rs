fn main() {
    let str = "gsv jfrxp yildm ulc qfnkh levi gsv ozab wlt";
    let str = cipher(str);
    println!("{}", str);

    let str = cipher(&str);
    println!("{}", str);
}

fn cipher(str: &str) -> String {
    str.bytes().into_iter().map(|x| {
        return if x.is_ascii_lowercase() {
            (219 - x) as char
        } else {
            x as char
        };
    }).collect()
}