pub fn word_n_gram(str: &str, n: usize) -> Vec<Vec<&str>> {
    let chars: Vec<_> = str.split_whitespace().collect();
    // let f = |index| chars.iter().skip(index).take(n).copied().collect::<Vec<_>>();
    let f = |index| chars.iter().skip(index).take(n).map(|x| *x).collect::<Vec<_>>();
    let gram: Vec<_> = (0..chars.len() - n + 1).map(|index| f(index)).collect();
    return gram;
}

pub fn n_gram(str: &str, n: usize) -> Vec<&str> {
    let gram: Vec<_> = (0..str.len() - n + 1).map(|index| &str[index..index + n]).collect();
    return gram;
}
