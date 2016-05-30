use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/// 17. １列目の文字列の異なり
/// 1列目の文字列の種類（異なる文字列の集合）を求めよ．
/// 確認にはsort, uniqコマンドを用いよ．
/// diff -u <(cat data/hightemp.txt| cut -f 1 | sort -u) <(./target/debug/nlp17 data/hightemp.txt | sort )
pub fn main() {
    let filename = env::args().skip(1).next().unwrap();
    let f = File::open(filename).unwrap();
    let reader = BufReader::new(f);

    let mut col1 = std::collections::BTreeSet::<String>::new();

    for line in reader.lines().flat_map(|r| r.ok()) {
        let mut l = line.split_whitespace();
        col1.insert(l.next().unwrap().to_string());
    }

    for c in col1 {
        std::io::stdout().write_fmt(format_args!("{}\n", c)).unwrap();
    }

}
