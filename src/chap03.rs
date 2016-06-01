#[allow(unused_imports)]
use flate2::FlateReadExt;
#[allow(unused_imports)]
use std::fs::File;
#[allow(unused_imports)]
use std::io::{BufRead, BufReader};
#[allow(unused_imports)]
use regex::Regex;

#[allow(unused_imports)]
use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct Country {
    title: String,
    text: String,
}

#[allow(dead_code)]
pub fn get_country_text(country: &str) -> Option<String> {
    let f = File::open("data/jawiki-country.json.gz").unwrap();
    let decoder = BufReader::new(f).gz_decode().unwrap();
    let reader = BufReader::new(decoder);
    reader.lines()
        .flat_map(|r| r.ok())
        .map(|l| json::decode::<Country>(&l))
        .flat_map(|r| r.ok())
        .find(|j| j.title == country)
        .map(|j| j.text)
}

/// 20. JSONデータの読み込み
/// Wikipedia記事のJSONファイルを読み込み，「イギリス」に関する記事本
/// 文を表示せよ．問題21-29では，ここで抽出した記事本文に対して実行せ
/// よ．
#[test]
fn nlp20() {
    let r = get_country_text("イギリス");
    assert!(r.is_some());
}

/// 21. カテゴリ名を含む行を抽出
/// 記事中でカテゴリ名を宣言している行を抽出せよ．
#[test]
fn nlp21() {
    let text = get_country_text("イギリス").unwrap();
    let lines: Vec<&str> = text.split("\n").collect();
    let cats: Vec<&str> = lines.into_iter().filter(|l| l.contains("[[Category:")).collect();
    println!("{}\n", cats.join("\n"));
    assert_eq!(8, cats.len());
}
