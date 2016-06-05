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

/// 22. カテゴリ名の抽出
/// 記事のカテゴリ名を（行単位ではなく名前で）抽出せよ．
#[test]
fn nlp22() {
    let text = get_country_text("イギリス").unwrap();
    let re = Regex::new(r"\[\[Category:(.+?)\]\]").unwrap();
    let cats: Vec<_> = re.captures_iter(&text).flat_map(|c| c.at(1)).collect();
    for c in &cats {
        println!("{}", c);
    }
    assert_eq!(cats,
               vec!["イギリス|*",
                    "英連邦王国|*",
                    "G8加盟国",
                    "欧州連合加盟国",
                    "海洋国家",
                    "君主国",
                    "島国|くれいとふりてん",
                    "1801年に設立された州・地域"]);

}
