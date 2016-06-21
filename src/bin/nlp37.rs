extern crate nlp100rs;
extern crate itertools;

use itertools::Itertools;
use nlp100rs::chap04::*;
use std::process::{Command, Stdio};
use std::io::Write;


/// 37. 頻度上位10語
/// 出現頻度が高い10語とその出現頻度をグラフ（例えば棒グラフなど）で表示せよ．
pub fn main() {
    let neko = neko();
    let mut ms: Vec<Morph> = (&neko).concat();
    ms.sort_by(|a, b| a.surface.cmp(&b.surface));

    let mut hist: Vec<(Morph, usize)> = ms.into_iter()
        .group_by(|m| m.surface.clone())
        .map(|(_, v)| (v[0].clone(), v.len()))
        .collect();

    hist.sort_by_key(|&(_, l)| l);
    hist.reverse();

    let mut plot = Command::new("gnuplot")
        .arg("-")
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();
    let stdin = plot.stdin.as_mut().unwrap();
    stdin.write_fmt(format_args!(r#"
set term pngcairo
set output "nlp37.png"
plot "-" using 0:2:xticlabels(1) with boxes notitle
"#))
        .unwrap();

    for &(ref m, c) in hist.iter().take(10) {
        stdin.write_fmt(format_args!("{} {}\n", m.surface, c)).unwrap();
    }

    stdin.write_fmt(format_args!("EOF\n")).unwrap();
    stdin.flush().unwrap();;
}
