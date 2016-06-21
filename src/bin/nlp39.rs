extern crate nlp100rs;
extern crate itertools;

use itertools::Itertools;
use nlp100rs::chap04::*;
use std::process::{Command, Stdio};
use std::io::Write;

/// 39. Zipfの法則
/// 単語の出現頻度順位を横軸，その出現頻度を縦軸として，両対数グラフを
/// プロットせよ．
pub fn main() {
    let neko = neko();
    let mut ms: Vec<Morpheme> = (&neko).concat();
    ms.sort_by(|a, b| a.surface.cmp(&b.surface));

    let mut hist: Vec<(Morpheme, usize)> = ms.into_iter()
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
set output "nlp39.png"
set logscale
set yrange[1:10000]
plot "-" using 0:2 with lines notitle
"#))
        .unwrap();

    for &(ref m, c) in hist.iter() {
        stdin.write_fmt(format_args!("{} {}\n", m.surface, c)).unwrap();
    }

    stdin.write_fmt(format_args!("EOF\n")).unwrap();
    stdin.flush().unwrap();;
}
