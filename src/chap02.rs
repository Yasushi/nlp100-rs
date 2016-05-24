/*
hightemp.txtは，日本の最高気温の記録を「都道府県」「地点」「℃」「日」
のタブ区切り形式で格納したファイルである．以下の処理を行うプログラムを
作成し，hightemp.txtを入力ファイルとして実行せよ．さらに，同様の処理を
UNIXコマンドでも実行し，プログラムの実行結果を確認せよ．
 */


#[allow(unused_imports)]
use std::fs::File;
#[allow(unused_imports)]
use std::io::{BufRead, BufReader};


/// 10. 行数のカウント
#[test]
fn nlp10() {
    let f = File::open("data/hightemp.txt").unwrap();
    let reader = BufReader::new(f);
    let count = reader.lines().count();
    println!("lines {}", count);
    assert!(count == 24);
}
