use lang_lib::func::n_gram;
use std::collections::HashSet;

fn main() {
    let str_x = "paraparaparadise";
    let str_y = "paragraph";
    let set_x: HashSet<_> = to_hashset(n_gram(str_x, 2));
    let set_y: HashSet<_> = to_hashset(n_gram(str_y, 2));

    // 和集合
    let set: HashSet<_> = set_x.union(&set_y).collect();
    let set_and: HashSet<_> = set_x.intersection(&set_y).collect();
    let set_x_y: HashSet<_> = set_x.difference(&set_y).collect();
    let set_y_x: HashSet<_> = set_y.difference(&set_x).collect();
    println!("x = {:?}", set_x);
    println!("y = {:?}", set_y);
    println!("x + y = {:?}", set);
    println!("x & y = {:?}", set_and);
    println!("x - y = {:?}", set_x_y);
    println!("y - x = {:?}", set_y_x);

    println!("x & [se] = {:?}", set_x.contains("se"));
    println!("y & [se] = {:?}", set_y.contains("se"));
}

fn to_hashset(vec: Vec<&str>) -> HashSet<&str> {
    vec.iter().copied().collect()
}
