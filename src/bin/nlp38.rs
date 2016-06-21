extern crate nlp100rs;
extern crate itertools;

use itertools::Itertools;
use nlp100rs::chap04::*;
use std::process::{Command, Stdio};
use std::io::Write;

/// 38. ヒストグラム
/// 単語の出現頻度のヒストグラム（横軸に出現頻度，縦軸に出現頻度をとる
/// 単語の種類数を棒グラフで表したもの）を描け．
pub fn main() {
    let neko = neko();
    let mut ms: Vec<Morph> = (&neko).concat();
    ms.sort_by(|a, b| a.surface.cmp(&b.surface));

    let mut wordhist: Vec<(String, usize)> = ms.into_iter()
        .group_by(|m| m.surface.clone())
        .map(|(word, v)| (word, v.len()))
        .collect();

    wordhist.sort_by_key(|&(_, count)| count);

    let mut hist: Vec<(usize, usize)> = wordhist.into_iter()
        .group_by(|&(_, count)| count)
        .map(|(count, ws)| (count, ws.len()))
        .collect();

    hist.sort_by_key(|&(count, _)| count);

    let mut plot = Command::new("gnuplot")
        .arg("-")
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();
    let stdin = plot.stdin.as_mut().unwrap();
    stdin.write_fmt(format_args!(r#"
set term pngcairo size 1000,480
set output "nlp38.png"
plot "-" using 1:2 with boxes notitle
"#))
        .unwrap();

    for &(count, words) in hist.iter().take(100) {
        stdin.write_fmt(format_args!("{} {}\n", count, words)).unwrap();
    }

    stdin.write_fmt(format_args!("EOF\n")).unwrap();
    stdin.flush().unwrap();;
}
