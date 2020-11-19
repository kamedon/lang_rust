use rand::thread_rng;
use rand::seq::SliceRandom;

fn main() {
    let str = "I couldn’t believe that I could actually understand what I was reading : the phenomenal power of the human mind .";


    let ans: Vec<_> = str.split_whitespace().into_iter().map(|x| {
        if x.len() >= 4 {
            let mut rand_chars: Vec<_> = x[1..x.len() - 1].chars().collect();
            let mut rng = thread_rng();
            rand_chars.shuffle(&mut rng);
            let str: String = rand_chars.iter().collect();
            let chars: Vec<_> = x.chars().collect();
            format!("{}{}{}", chars.first().unwrap(), str, chars.last().unwrap())
        } else {
            String::from(x)
        }
    }).collect();
    println!("{:?}", ans);
}