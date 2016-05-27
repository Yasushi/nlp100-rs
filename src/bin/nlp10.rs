use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;

/// 10. 行数のカウント
fn main() {
    let filename = env::args().skip(1).next().unwrap();
    let f = File::open(filename).unwrap();
    let reader = BufReader::new(f);
    let count = reader.lines().count();
    println!("{}", count);
}
