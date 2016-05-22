extern crate rand;

#[allow(unused_imports)]
use std::iter::FromIterator;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::fmt::Display;

use self::rand::Rng;


/// 00. 文字列の逆順
#[allow(dead_code)]
pub fn reverse(s: &str) -> String {
    String::from_iter(s.chars().rev().collect::<Vec<_>>())
}

// 文字列"stressed"の文字を逆に（末尾から先頭に向かって）並べた文字列を得よ．
#[test]
pub fn nlp00() {
    assert!(String::from_iter("stressed".chars().rev().collect::<Vec<_>>()) == "desserts");
    assert!(reverse("stressed") == "desserts");
}

/// 01. 「パタトクカシーー」
///
/// 「パタトクカシーー」という文字列の1,3,5,7文字目を取り出して連結した文字列を得よ．
#[test]
pub fn nlp01() {
    let s = "パタトクカシーー";
    let r = String::from_iter(s.char_indices().filter(|&(i, _)| i % 2 == 0).map(|(_, s)| s));
    println!("r={}", r);
    assert!(r == "パトカー");
}

/// 02. 「パトカー」＋「タクシー」＝「パタトクカシーー」
///
///「パトカー」＋「タクシー」の文字を先頭から交互に連結して文字列「パタトクカシーー」を得よ．
#[test]
pub fn nlp02() {
    let p = "パトカー";
    let t = "タクシー";
    let r = String::from_iter(p.chars().zip(t.chars()).flat_map(|(a, b)| vec![a, b]));
    assert!(r == "パタトクカシーー");
}

/// 03. 円周率
/// 
/// "Now I need a drink, alcoholic of course, after the heavy lectures involving quantum mechanics." 
/// という文を単語に分解し，各単語の（アルファベットの）文字数を先頭から出現順に並べたリストを作成せよ．
#[test]
pub fn nlp03() {
    let s = "Now I need a drink, alcoholic of course, after the heavy lectures involving quantum \
             mechanics.";
    let cs: Vec<_> = s.split_whitespace()
                      .map(|s| s.chars().filter(|&c| char::is_alphabetic(c)).count())
                      .collect();
    println!("cs = {:?}", cs);
    assert!(cs == vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9]);
}

/// 04. 元素記号
///
/// "Hi He Lied Because Boron Could Not Oxidize Fluorine. New Nations Might Also Sign Peace Security Clause. Arthur King Can."
/// 
/// という文を単語に分解し，1, 5, 6, 7, 8, 9, 15, 16, 19番目の単語は先
/// 頭の1文字，それ以外の単語は先頭に2文字を取り出し，取り出した文字列
/// から単語の位置（先頭から何番目の単語か）への連想配列（辞書型もしく
/// はマップ型）を作成せよ．
#[test]
pub fn nlp04() {
    let s = "Hi He Lied Because Boron Could Not Oxidize Fluorine. New Nations Might Also Sign \
             Peace Security Clause. Arthur King Can.";
    let single = vec![1, 5, 6, 7, 8, 9, 15, 16, 19];
    let m: HashMap<usize, String> = s.split_whitespace()
                                     .zip(1..)
                                     .map(|(w, i)| if single.contains(&i) {
                                         (i, w.chars().next().unwrap().to_string())
                                     } else {
                                         (i, String::from_iter(w.chars().take(2)))
                                     })
                                     .collect();
    println!("m = {:?}", m);
    assert!(m.get(&1usize) == Some(&"H".to_string()));
    assert!(m.get(&2usize) == Some(&"He".to_string()));
    assert!(m.get(&3usize) == Some(&"Li".to_string()));
    assert!(m.get(&4usize) == Some(&"Be".to_string()));
    assert!(m.get(&5usize) == Some(&"B".to_string()));
    assert!(m.get(&6usize) == Some(&"C".to_string()));
    assert!(m.get(&7usize) == Some(&"N".to_string()));
    assert!(m.get(&8usize) == Some(&"O".to_string()));
    assert!(m.get(&9usize) == Some(&"F".to_string()));
    assert!(m.get(&10usize) == Some(&"Ne".to_string()));
    assert!(m.get(&11usize) == Some(&"Na".to_string()));
    assert!(m.get(&12usize) == Some(&"Mi".to_string())); // Mg
    assert!(m.get(&13usize) == Some(&"Al".to_string()));
    assert!(m.get(&14usize) == Some(&"Si".to_string()));
    assert!(m.get(&15usize) == Some(&"P".to_string()));
    assert!(m.get(&16usize) == Some(&"S".to_string()));
    assert!(m.get(&17usize) == Some(&"Cl".to_string()));
    assert!(m.get(&18usize) == Some(&"Ar".to_string()));
    assert!(m.get(&19usize) == Some(&"K".to_string()));
    assert!(m.get(&20usize) == Some(&"Ca".to_string()));
}

/// 05. n-gram
///
/// 与えられたシーケンス（文字列やリストなど）からn-gramを作る関数を作成せよ．この関数を用い，"I am an NLPer"という文から単語bi-gram，文字bi-gramを得よ．
#[allow(dead_code)]
pub fn ngram<E: Clone, T: Iterator<Item = E>>(it: &mut T, width: usize) -> Vec<Vec<E>> {
    let mut window: Vec<E> = Vec::with_capacity(width + 1);

    for _ in 0..width {
        if let Some(e) = it.next() {
            window.push(e);
        }
    }

    if window.len() < width {
        return vec![window];
    }

    let mut result: Vec<Vec<E>> = vec![window.clone()];
    loop {
        if let Some(e) = it.next() {
            window.push(e);
            window.remove(0);
            result.push(window.clone());
        } else {
            return result;
        }
    }
}

#[test]
fn nlp05() {
    let s = "I am an NLPer";
    let words = ngram(&mut s.split_whitespace(), 2);
    let chars: Vec<String> = ngram(&mut s.chars(), 2)
                                 .into_iter()
                                 .map(|v: Vec<char>| String::from_iter(v))
                                 .collect();
    println!("words {:?}", words);
    println!("chars {:?}", chars);
    assert!(words == vec![vec!["I", "am"], vec!["am", "an"], vec!["an", "NLPer"]]);
    assert!(chars == vec!["I ", " a", "am", "m ", " a", "an", "n ", " N", "NL", "LP", "Pe", "er"]);
}

#[allow(dead_code)]
pub fn ngram2<E: Clone>(it: &[E], width: usize) -> Vec<Vec<E>> {
    it.windows(width).map(|es| es.to_vec()).collect()
}

#[test]
fn nlp05_slice() {
    let s = "I am an NLPer";
    let words = ngram2(&mut s.split_whitespace().collect::<Vec<_>>().as_ref(), 2);
    let chars: Vec<String> = ngram(&mut s.chars(), 2)
                                 .into_iter()
                                 .map(|v: Vec<char>| String::from_iter(v))
                                 .collect();
    println!("words {:?}", words);
    println!("chars {:?}", chars);
    assert!(words == vec![vec!["I", "am"], vec!["am", "an"], vec!["an", "NLPer"]]);
    assert!(chars == vec!["I ", " a", "am", "m ", " a", "an", "n ", " N", "NL", "LP", "Pe", "er"]);
}


/// 06. 集合
///
/// "paraparaparadise"と"paragraph"に含まれる文字bi-gramの集合を，それ
/// ぞれ, XとYとして求め，XとYの和集合，積集合，差集合を求めよ．さら
/// に，'se'というbi-gramがXおよびYに含まれるかどうかを調べよ．
#[test]
fn nlp06() {
    let x = "paraparaparadise";
    let y = "paragraph";
    let xs: HashSet<_> = ngram(&mut x.chars(), 2)
                             .into_iter()
                             .map(|v: Vec<char>| String::from_iter(v))
                             .collect();
    let ys: HashSet<_> = ngram(&mut y.chars(), 2)
                             .into_iter()
                             .map(|v: Vec<char>| String::from_iter(v))
                             .collect();
    let union: HashSet<&String> = xs.union(&ys).collect();
    println!("{:?}", union);
    for &e in &["di", "se", "ad", "ph", "ar", "ap", "pa", "is", "ag", "ra", "gr"] {
        assert!(union.contains(&e.to_string()));
    }
    assert!(union.len() == 11);
    let inter: HashSet<_> = xs.intersection(&ys).collect();
    println!("{:?}", inter);
    for &e in &["ar", "ap", "pa", "ra"] {
        assert!(inter.contains(&e.to_string()));
    }
    assert!(inter.len() == 4);
    let diff: HashSet<_> = xs.difference(&ys).collect();
    println!("{:?}", diff);
    for &e in &["di", "se", "ad", "is"] {
        assert!(diff.contains(&e.to_string()));
    }
    assert!(diff.len() == 4);
}

/// 07. テンプレートによる文生成
///
/// 引数x, y, zを受け取り「x時のyはz」という文字列を返す関数を実装せよ．
#[allow(dead_code)]
pub fn template<X, Y, Z>(x: X, y: Y, z: Z) -> String
    where X: Display, Y: Display, Z: Display {
    format!("{}時の{}は{}", x, y, z)
}

// さらに，x=12, y="気温", z=22.4として，実行結果を確認せよ．
#[test]
fn nlp07() {
    assert!(template(12, "気温", 22.4) == "12時の気温は22.4");
}

/// 08. 暗号文
///
/// 与えられた文字列の各文字を，以下の仕様で変換する関数cipherを実装せよ．
/// * 英小文字ならば(219 - 文字コード)の文字に置換
/// * その他の文字はそのまま出力
#[allow(dead_code)]
pub fn cipher(s: &str) -> String {
    String::from_iter(
        s.chars().map(|c|
                      if c.is_lowercase() {
                          ::std::char::from_u32(219 - c as u32).unwrap()
                      } else {
                          c
                      })
    )
}

#[test]
fn nlp08() {
    let s = "I am an NLPer";
    println!("{}", cipher(s));
    assert!(cipher(s) == "I zn zm NLPvi");
    assert!(cipher("I zn zm NLPvi") == s);
}


/// 09. Typoglycemia
///
/// スペースで区切られた単語列に対して，各単語の先頭と末尾の文字は残し，
/// それ以外の文字の順序をランダムに並び替えるプログラムを作成せよ．た
/// だし，長さが４以下の単語は並び替えないこととする．適当な英語の文
/// （例えば"I couldn't believe that I could actually understand what
/// I was reading : the phenomenal power of the human mind ."）を与え，
/// その実行結果を確認せよ．
#[allow(dead_code)]
pub fn shuffle_word(s: &str) -> String {
    
    match s.len() {
        len if len <= 4 => s.to_string(),
        len => {
            let cs: Vec<_> = s.to_string().chars().collect();
            let mut i: Vec<_> = cs.clone();
            i.remove(len-1);
            i.remove(0);
            rand::thread_rng().shuffle(&mut i);
            i.insert(0, *cs.first().unwrap());
            i.push(*cs.last().unwrap());
            String::from_iter(i)
        }
    }
}

#[test]
fn nlp09() {
    let s = "I couldn't believe that I could actually understand what I was reading : the phenomenal power of the human mind .";
    let t = s.split_whitespace().map(shuffle_word).collect::<Vec<_>>().join(" ");
    println!("{}", t);
    assert!(true);
}
