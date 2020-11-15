fn main() {
    let str = "パタトクカシーー";
    let indexs = [0, 2, 4, 6];
    let ans: String = str.chars().into_iter()
        .enumerate()
        .filter(|(index,_)| {
            indexs.contains(&index)
        })
        .map(|x| x.1)
        .collect();
    println!("{}", ans)
}