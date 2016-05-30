use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::str::FromStr;

/// 16. ファイルをN分割する
/// 自然数Nをコマンドライン引数などの手段で受け取り，入力のファイルを
/// 行単位でN分割せよ．同様の処理をsplitコマンドで実現せよ．
pub fn main() {
    let mut args = env::args().skip(1);
    let n = args.next().and_then(|s| usize::from_str(s.as_str()).ok()).unwrap();
    let filename = args.next().unwrap();
    let reader = File::open(filename).map(BufReader::new).unwrap();
    let lines: Vec<String> = reader.lines().flat_map(|l| l.ok()).collect();
    let unit = lines.len() / n;

    for (i, c) in lines.chunks(unit).enumerate() {
        let mut out = File::create(format!("target/nlp16_{}.txt", i)).unwrap();
        out.write_fmt(format_args!("{}\n", c.join("\n"))).unwrap();
    }
}
