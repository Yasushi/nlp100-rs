extern crate itertools;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use itertools::Itertools;

/// 19. 各行の1コラム目の文字列の出現頻度を求め，出現頻度の高い順に並べる
/// 各行の1列目の文字列の出現頻度を求め，その高い順に並べて表示せよ．
/// 確認にはcut, uniq, sortコマンドを用いよ．
pub fn main() {
    let filename = env::args().skip(1).next().unwrap();
    let f = File::open(filename).unwrap();
    let reader = BufReader::new(f);

    let mut col1: Vec<String> = Vec::new();

    for line in reader.lines().flat_map(|r| r.ok()) {
        let mut l = line.split_whitespace();
        col1.push(l.next().unwrap().to_string());
        println!("{}", col1.join(","));
    }
    col1.sort(); // itertools::GroupBy はソート済みでなければならない

    let mut hist: Vec<(String, usize)> = col1.into_iter()
        .group_by(|e| e.to_string())
        .map(|(k, v)| (k, v.len()))
        .collect();
    println!("{}", hist.len());

    hist.sort_by_key(|&(_, l)| l);
    println!("{:?}", hist);

    let mut out = std::io::stdout();
    for (s, _) in hist.into_iter().rev() {
        out.write_fmt(format_args!("{}\n", s)).unwrap();
    }
}
