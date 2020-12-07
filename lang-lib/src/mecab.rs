#[derive(Debug)]
pub struct Morpheme<'a> {
    //表層系
    pub surface: &'a str,
    //基本形
    pub base: &'a str,
    //品詞
    pub pos: &'a str,
    //品詞細分類1
    pub pos1: &'a str,
}

impl<'a> Morpheme<'a>{
    pub fn new(line: &'a str) -> Morpheme<'a> {
        let mut split_line = line.split("\t");
        let surface = split_line.next().unwrap();
        let mut other = split_line.next().unwrap().split(",");
        let pos = other.next().unwrap();
        let pos1 = other.next().unwrap();
        let base = other.skip(4).next().unwrap();

        Morpheme {
            surface,
            pos,
            pos1,
            base,
        }

    }
}

#[derive(Debug)]
pub struct Sentence<'a> {
    pub morphemes: Vec<Morpheme<'a>>
}


pub fn convert_sentences(text: &str) -> Vec<Sentence> {
    let mut sentences: Vec<Sentence> = vec![Sentence { morphemes: vec![] }];
    for line in text.lines() {
        if line == "EOS" {
            sentences.push(Sentence {
                morphemes: vec![]
            });
            continue;
        }
        let morpheme = Morpheme::new(&line);
        sentences.last_mut().unwrap().morphemes.push(morpheme)
    }
    sentences
}

