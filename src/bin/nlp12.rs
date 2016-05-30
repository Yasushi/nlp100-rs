use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/// 12. 1列目をcol1.txtに，2列目をcol2.txtに保存
pub fn main() {
    let filename = env::args().skip(1).next().unwrap();
    let f = File::open(filename).unwrap();
    let reader = BufReader::new(f);
    let mut col1 = Vec::<String>::new();
    let mut col2 = Vec::<String>::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            let mut l = line.split_whitespace();
            col1.push(l.next().unwrap().to_owned());
            col2.push(l.next().unwrap().to_owned());
        }
    }

    let mut f1 = File::create("target/col1.txt").unwrap();
    f1.write_fmt(format_args!("{}\n", col1.join("\n"))).unwrap();

    let mut f2 = File::create("target/col2.txt").unwrap();
    f2.write_fmt(format_args!("{}\n", col2.join("\n"))).unwrap();
    // diff -u <(cut -f 1 data/hightemp.txt) target/col1.txt
    // diff -u <(cut -f 2 data/hightemp.txt) target/col2.txt
}
