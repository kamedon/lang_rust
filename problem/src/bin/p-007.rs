fn main() {
    let str = template(12, "気温", 22.4);
    println!("{}", str)
}

fn template(x: u32, y: &str, z: f64) -> String {
    return format!("{}時の{}は{}", x, y, z);
}