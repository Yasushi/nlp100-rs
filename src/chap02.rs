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

/// 12. 1列目をcol1.txtに，2列目をcol2.txtに保存
///
/// 各行の1列目だけを抜き出したものをcol1.txtに，2列目だけを抜き出したものをcol2.txtとしてファイルに保存せよ．確認にはcutコマンドを用いよ．
#[test]
fn nlp12() {
    let f = File::open(HIGHTEMP).unwrap();
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
    f1.write_fmt(format_args!("{}\n",col1.join("\n"))).unwrap();

    let mut f2 = File::create("target/col2.txt").unwrap();
    f2.write_fmt(format_args!("{}\n",col2.join("\n"))).unwrap();
    // diff -u <(cut -f 1 data/hightemp.txt) target/col1.txt
    // diff -u <(cut -f 2 data/hightemp.txt) target/col2.txt
    assert!(true);
}


/// 13. col1.txtとcol2.txtをマージ
///
/// 12で作ったcol1.txtとcol2.txtを結合し，元のファイルの1列目と2列目をタブ区切りで並べたテキストファイルを作成せよ．確認にはpasteコマンドを用いよ．
#[test]
fn nlp13() {
    let f1 = File::open("target/col1.txt").unwrap();
    let col1:Vec<String> = BufReader::new(f1).lines().map(Result::unwrap).collect();

    let f2 = File::open("target/col2.txt").unwrap();
    let col2:Vec<String> = BufReader::new(f2).lines().map(Result::unwrap).collect();

    let zip = col1.iter().zip(col2);

    let mut out = File::create("target/paste.txt").unwrap();
    for t in zip {
        out.write_fmt(format_args!("{}\t{}\n", t.0, t.1)).unwrap();
    }
    // diff -u <(paste target/col1.txt target/col2.txt) target/paste.txt
    assert!(true);
}
