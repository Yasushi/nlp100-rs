use std::env;
use std::fs::File;
use std::io::{Read,BufReader,Write};


/// 11. タブをスペースに置換
pub fn main() {
    let filename = env::args().skip(1).next().unwrap();
    let f = File::open(filename).unwrap();
    let mut reader = BufReader::new(f);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).unwrap();
    let replaced = contents.as_str().replace("\t", " ");
    std::io::stdout().write_all(replaced.as_bytes()).unwrap();
    // diff -u <(expand -t 1 data/hightemp.txt ) <(./target/debug/nlp11 data/hightemp.txt)
}
