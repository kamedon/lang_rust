use rand::thread_rng;
use rand::seq::SliceRandom;

fn main() {
    let str = "I couldn’t believe that I could actually understand what I was reading : the phenomenal power of the human mind .";


    let ans: Vec<_> = str.split_whitespace().into_iter().map(|x| {
        return if x.len() >= 4 {
            let mut rand_chars: Vec<_> = x.chars().enumerate().filter(|(index, _)| *index != 0 && *index != x.len() - 1).map(|(_, value)| {
                value
            }).collect();
            let mut rng = thread_rng();
            rand_chars.shuffle(&mut rng);
            let str: String = rand_chars.iter().collect();
            let first = x.chars().next().unwrap();
            let last = x.chars().last().unwrap();
            let str = format!("{}{}{}", first, str, last);

            str
        } else {
            String::from(x)
        };
    }).collect();
    println!("{:?}", ans);
}