use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/// 13. col1.txtとcol2.txtをマージ
pub fn main() {
    let mut args = env::args().skip(1);

    let n1 = args.next().unwrap();
    let f1 = File::open(n1).unwrap();
    let col1: Vec<String> = BufReader::new(f1).lines().map(Result::unwrap).collect();

    let n2 = args.next().unwrap();
    let f2 = File::open(n2).unwrap();
    let col2: Vec<String> = BufReader::new(f2).lines().map(Result::unwrap).collect();

    let zip = col1.iter().zip(col2);

    let mut out = std::io::stdout();
    for t in zip {
        out.write_fmt(format_args!("{}\t{}\n", t.0, t.1)).unwrap();
    }
    // diff -u <(paste target/col1.txt target/col2.txt ) <(./target/debug/nlp13 target/col1.txt target/col2.txt )
}
