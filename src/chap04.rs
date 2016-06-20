use std::fs::File;
use std::fmt;
use std::io::{BufRead, BufReader};
#[allow(unused_imports)]
use itertools::Itertools;

#[derive(Debug, PartialEq, Clone)]
pub struct Morpheme {
    pub surface: String,
    pub base: String,
    pub pos: String,
    pub pos1: String,
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
    }
}

impl fmt::Display for Morpheme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{},{},{},{}",
               self.surface,
               self.base,
               self.pos,
               self.pos1)
    }
}

pub type Sentence = Vec<Morpheme>;

pub fn neko() -> Vec<Sentence> {
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
    let neko = neko();
    assert_eq!(neko[0][0], Morpheme::new("一", "一", "名詞", "数"));
    assert!(neko[1].is_empty());
    assert_eq!(neko[2][1],
               Morpheme::new("吾輩", "吾輩", "名詞", "代名詞"));
}

/// 31. 動詞
/// 動詞の表層形をすべて抽出せよ．
#[test]
fn nlp31() {
    let neko = neko();
    let verbs: Vec<Morpheme> = neko.into_iter().flat_map(|s| s.into_iter().filter(|m| m.pos == "動詞")).collect();
    for v in &verbs {
        println!("{}", v.surface);
    }
    assert_eq!(verbs.len(), 28906);
}

/// 32. 動詞の原形
/// 動詞の原形をすべて抽出せよ．
#[test]
fn nlp32() {
    let neko = neko();
    let verbs: Vec<Morpheme> = neko.into_iter().flat_map(|s| s.into_iter().filter(|m| m.pos == "動詞")).collect();
    for v in &verbs {
        println!("{}", v.base);
    }
    assert_eq!(verbs.len(), 28906);
}

/// 33. サ変名詞
/// サ変接続の名詞をすべて抽出せよ．
#[test]
fn nlp33() {
    let neko = neko();
    let noun: Vec<Morpheme> = neko.into_iter()
        .flat_map(|s| {
            s.into_iter()
                .filter(|m| m.pos == "名詞" && m.pos1 == "サ変接続")
        })
        .collect();
    for v in &noun {
        println!("{}", v);
    }
    assert_eq!(noun.len(), 5209);
}

/// 34. 「AのB」
/// 2つの名詞が「の」で連結されている名詞句を抽出せよ．
#[test]
fn nlp34() {
    let neko = neko();
    let triplets: Vec<Vec<Morpheme>> = neko.iter()
        .flat_map(|s| s.windows(3).map(|w| w.to_vec()))
        .filter(|t| t[1].surface == "の" && t[0].pos == "名詞" && t[2].pos == "名詞")
        .collect();
    for t in &triplets {
        println!("{}{}{}", t[0].surface, t[1].surface, t[2].surface);
    }
    assert_eq!(triplets.len(), 6044);
}


/// 35. 名詞の連接
/// 名詞の連接（連続して出現する名詞）を最長一致で抽出せよ．
#[test]
fn nlp35() {
    let neko = neko();
    let mut runs: Vec<Vec<Morpheme>> = Vec::new();
    for s in neko {
        let mut ns: Vec<Morpheme> = Vec::new();
        for v in s {
            if v.pos == "名詞" {
                ns.push(v);
            } else {
                if !ns.is_empty() {
                    runs.push(ns.clone());
                    ns.clear();
                }
            }
        }
    }
    let runs: Vec<Vec<Morpheme>> = runs.into_iter().filter(|r| r.len() > 1).collect();
    for r in &runs {
        for m in r {
            print!("{} ", m.surface);
        }
        println!("");
    }
    assert_eq!(runs.len(), 7335);
}

/// 36. 単語の出現頻度
/// 文章中に出現する単語とその出現頻度を求め，出現頻度の高い順に並べよ．
#[test]
fn nlp36() {
    let neko = neko();
    let mut ms: Vec<Morpheme> = (&neko).concat();
    ms.sort_by(|a, b| a.surface.cmp(&b.surface));

    let mut hist: Vec<(Morpheme, usize)> = ms.into_iter()
        .group_by(|m| m.surface.clone())
        .map(|(_, v)| (v[0].clone(), v.len()))
        .collect();

    hist.sort_by_key(|&(_, l)| l);
    hist.reverse();

    for &(ref m, c) in &hist {
        println!("{} {}", m.surface, c);
    }
    assert_eq!(hist[0].0.surface, "の");
    assert_eq!(hist[0].1, 9194);
    assert_eq!(hist.len(), 13584);
}
