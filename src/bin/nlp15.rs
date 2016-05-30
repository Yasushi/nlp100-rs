use std::env;
use std::str::FromStr;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/// 15. 末尾のN行を出力
///
/// 自然数Nをコマンドライン引数などの手段で受け取り，入力のうち末尾の
/// N 行だけを表示せよ．確認にはtailコマンドを用いよ．
pub fn main() {
    let mut args = env::args().skip(1);
    let n = args.next().and_then(|s| usize::from_str(s.as_str()).ok()).unwrap();
    let filename = args.next().unwrap();
    let reader = File::open(filename).map(BufReader::new).unwrap();
    let lines = reader.lines();

    let v = lines.flat_map(|r| r.ok()).collect::<Vec<_>>();
    let rev = v.iter().rev().take(n).collect::<Vec<_>>();

    let mut out = std::io::stdout();
    for l in rev.iter().rev() {
        out.write_fmt(format_args!("{}\n", l)).unwrap();
    }
    // diff -u <(tail -n 12 data/hightemp.txt ) <( ./target/debug/nlp15 12 data/hightemp.txt )
}
