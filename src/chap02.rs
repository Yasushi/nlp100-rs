/*
hightemp.txtは，日本の最高気温の記録を「都道府県」「地点」「℃」「日」
のタブ区切り形式で格納したファイルである．以下の処理を行うプログラムを
作成し，hightemp.txtを入力ファイルとして実行せよ．さらに，同様の処理を
UNIXコマンドでも実行し，プログラムの実行結果を確認せよ．
 */


#[allow(unused_imports)]
use std::fs::File;
#[allow(unused_imports)]
use std::io::{BufRead, BufReader, Read, Write};

#[allow(dead_code)]
const HIGHTEMP: &'static str = "data/hightemp.txt";

/// 10. 行数のカウント
#[test]
fn nlp10() {
    let f = File::open(HIGHTEMP).unwrap();
    let reader = BufReader::new(f);
    let count = reader.lines().count();
    println!("lines {}", count);
    assert!(count == 24);
}

/// 11. タブをスペースに置換
///
/// タブ1文字につきスペース1文字に置換せよ．
#[test]
fn nlp11() {
    let f = File::open(HIGHTEMP).unwrap();
    let mut reader = BufReader::new(f);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).unwrap();
    let replaced = contents.as_str().replace("\t", " ");
    let mut f = File::create("target/nlp11.txt").unwrap();
    f.write_all(replaced.as_bytes()).unwrap();
    // diff -u target/nlp11.txt <( expand -t 1 data/hightemp.txt)
    assert!(true);
}
