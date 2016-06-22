use chap04::Morph;

use std::collections::HashMap;
use std::iter::FromIterator;
use std::slice;
use std::fs::File;
use std::fmt;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::ops::Add;
use itertools::Itertools;

#[derive(Debug, PartialEq, Clone)]
pub struct Chunk {
    pub no: usize,
    pub morphs: Vec<Morph>,
    pub dst: isize,
    pub srcs: Vec<usize>,
}

impl Chunk {
    fn new() -> Chunk {
        Chunk {
            no: 0,
            morphs: Vec::new(),
            dst: -1,
            srcs: Vec::new(),
        }
    }

    fn add_morph(&mut self, m: Morph) {
        self.morphs.push(m);
    }

    fn is_empty(&self) -> bool {
        self.morphs.is_empty()
    }

    #[allow(dead_code)]
    fn join(&self) -> String {
        let mut result = String::new();
        for m in &self.morphs {
            result = result.add(&m.surface.clone());
        }
        result
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "* {} {} {:?}\n", self.no, self.dst, self.srcs));
        for m in &self.morphs {
            try!(write!(f, "{}", m.surface));
        }
        write!(f, "\n")
    }
}

pub struct Sentence(Vec<Chunk>);

impl Sentence {
    fn new() -> Sentence {
        Sentence(Vec::new())
    }

    fn relations(&mut self) {
        let mut rs: Vec<(usize, isize)> = self.0
            .iter()
            .map(|c| (c.no, c.dst))
            .filter(|&(_, dst)| dst >= 0)
            .collect();
        rs.sort_by_key(|&(_, dst)| dst);
        let relmap: HashMap<usize, Vec<usize>> = HashMap::from_iter(rs.into_iter()
            .group_by(|&(_, dst)| dst)
            .map(|(dst, ts)| (dst as usize, ts.iter().map(|t| t.0).collect::<Vec<usize>>())));
        for mut c in &mut self.0 {
            if let Some(srcs) = relmap.get(&c.no) {
                c.srcs.extend_from_slice(&srcs);
            }
        }
    }

    fn push(&mut self, c: Chunk) {
        self.0.push(c);
    }

    #[allow(dead_code)]
    fn iter(&self) -> slice::Iter<Chunk> {
        self.0.iter()
    }

    #[allow(dead_code)]
    fn pair(&self) -> Vec<(&Chunk, &Chunk)> {
        let mut result = Vec::new();
        for c in self.iter() {
            if c.dst >= 0 {
                result.push((c, &self.0[c.dst as usize]));
            }
        }
        result
    }
}

#[allow(dead_code)]
pub fn neko() -> Vec<Sentence> {
    let f = File::open("data/neko.txt.cabocha").unwrap();
    let mut result: Vec<Sentence> = Vec::new();
    let mut s = Sentence::new();
    let mut c = Chunk::new();
    for line in BufReader::new(f).lines() {
        match line {
            Ok(ref l) if l == "EOS" => {
                if !c.is_empty() {
                    s.push(c);
                }
                c = Chunk::new();
                s.relations();
                result.push(s);
                s = Sentence::new();
            }
            Ok(ref l) if l.starts_with("* ") => {
                if !c.is_empty() {
                    s.push(c);
                }
                c = Chunk::new();
                let mut ls = l.split(" ").skip(1);
                c.no = usize::from_str(ls.next().unwrap()).unwrap();
                let mut dst = ls.next().unwrap().to_string();
                dst.pop().unwrap();
                c.dst = isize::from_str(&dst).unwrap();
            }
            Ok(l) => {
                c.add_morph(Morph::build(&l));
            }
            _ => unreachable!(),
        }
    }
    return result;
}

/// 40. 係り受け解析結果の読み込み（形態素）
/// 形態素を表すクラスMorphを実装せよ．このクラスは表層形（surface），
/// 基本形（base），品詞（pos），品詞細分類1（pos1）をメンバ変数に持つ
/// こととする．さらに，CaboChaの解析結果（neko.txt.cabocha）を読み込
/// み，各文をMorphオブジェクトのリストとして表現し，3文目の形態素列を
/// 表示せよ．
///
/// 41. 係り受け解析結果の読み込み（文節・係り受け）
/// 40に加えて，文節を表すクラスChunkを実装せよ．このクラスは形態素
/// （Morphオブジェクト）のリスト（morphs），係り先文節インデックス番
/// 号（dst），係り元文節インデックス番号のリスト（srcs）をメンバ変数
/// に持つこととする．さらに，入力テキストのCaboChaの解析結果を読み込
/// み，１文をChunkオブジェクトのリストとして表現し，8文目の文節の文字
/// 列と係り先を表示せよ．第5章の残りの問題では，ここで作ったプログラ
/// ムを活用せよ．
#[test]
fn nlp41() {
    let neko = neko();
    for ref s in neko.iter().skip(7).take(1) {
        for c in s.iter() {
            print!("{}", c);
        }
        println!("");
    }
    assert!(false);
}

/// 42. 係り元と係り先の文節の表示
/// 係り元の文節と係り先の文節のテキストをタブ区切り形式ですべて抽出せ
/// よ．ただし，句読点などの記号は出力しないようにせよ．
#[test]
fn nlp42() {
    let neko = neko();
    for s in &neko {
        for (c1, c2) in s.pair() {
            println!("{}\t{}", c1.join(), c2.join());
        }
    }
}
