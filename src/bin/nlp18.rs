use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::str::FromStr;

fn key(line: &String) -> f32 {
    line.split_whitespace().skip(2).next().and_then(|s| f32::from_str(s).ok()).unwrap()
}

fn rcomp(x: &String, y: &String) -> std::cmp::Ordering {
    let x = (key(x) * 10.0) as i32;
    let y = (key(y) * 10.0) as i32;
    y.cmp(&x)
}

/// 18. 各行を3コラム目の数値の降順にソート
/// 各行を3コラム目の数値の逆順で整列せよ（注意: 各行の内容は変更せず
/// に並び替えよ）．確認にはsortコマンドを用いよ（この問題はコマンドで
/// 実行した時の結果と合わなくてもよい）．
pub fn main() {
    let filename = env::args().skip(1).next().unwrap();
    let f = File::open(filename).unwrap();
    let reader = BufReader::new(f);

    let mut lines = reader.lines()
        .flat_map(|l| l.ok())
        .collect::<Vec<String>>();
    // lines.sort_by_key(|l| (key(l) * 10.0) as i32);
    lines.sort_by(rcomp);

    for l in lines {
        std::io::stdout().write_fmt(format_args!("{}\n", l)).unwrap();
    }

}
