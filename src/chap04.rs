use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq)]
pub struct Morpheme {
    surface: String,
    base: String,
    pos: String,
    pos1: String,
}

impl Morpheme {
    fn new(surface: &str, base: &str, pos: &str, pos1: &str) -> Morpheme {
        Morpheme {
            surface: surface.to_string(),
            base: base.to_string(),
            pos: pos.to_string(),
            pos1: pos1.to_string(),
        }
    }
    fn build(line: &str) -> Morpheme {
        let s: Vec<&str> = line.split("\t").collect();
        let a: Vec<&str> = s[1].split(",").collect();
        Morpheme::new(s[0], a[6], a[0], a[1])
        // Morpheme{
        //     surface: s[0].to_string(),
        //     base: a[6].to_string(),
        //     pos: a[0].to_string(),
        //     pos1: a[1].to_string()
        // }
    }
}

pub type Sentence = Vec<Morpheme>;

pub fn get_neko() -> Vec<Sentence> {
    let f = File::open("data/neko.txt.mecab.ipa").unwrap();
    let mut result: Vec<Sentence> = Vec::new();
    let mut s: Sentence = Vec::new();
    for line in BufReader::new(f).lines() {
        match line {
            Ok(ref l) if l == "EOS" => {
                result.push(s);
                s = Vec::new();
            }
            Ok(l) => s.push(Morpheme::build(&l)),
            _ => unreachable!(),
        }
    }
    return result;
}


/// 30. 形態素解析結果の読み込み
/// 形態素解析結果（neko.txt.mecab）を読み込むプログラムを実装せよ．た
/// だし，各形態素は表層形（surface），基本形（base），品詞（pos），品
/// 詞細分類1（pos1）をキーとするマッピング型に格納し，1文を形態素（マッ
/// ピング型）のリストとして表現せよ．第4章の残りの問題では，ここで作っ
/// たプログラムを活用せよ
#[test]
fn nlp30() {
    let neko = get_neko();
    assert_eq!(neko[0][0], Morpheme::new("一", "一", "名詞", "数"));
    assert!(neko[1].is_empty());
    assert_eq!(neko[2][1],
               Morpheme::new("吾輩", "吾輩", "名詞", "代名詞"));
}
