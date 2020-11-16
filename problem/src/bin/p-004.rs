use std::collections::HashMap;

fn main() {
    let mut indexs = [1, 5, 6, 7, 8, 9, 15, 16, 19].iter().map(|x| x - 1).into_iter();
    let str = "Hi He Lied Because Boron Could Not Oxidize Fluorine. New Nations Might Also Sign Peace Security Clause. Arthur King Can.";
    let list = str.split_whitespace().enumerate().into_iter();
    let mut hash_map = HashMap::new();

    for (index, value) in list {
        if indexs.any(|x| x == index) {
            hash_map.insert(index, &value[0..1]);
        } else {
            hash_map.insert(index, &value[0..2]);
        }
    }
    println!("{:?}", hash_map);
}
