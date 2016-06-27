extern crate nlp100rs;
extern crate itertools;

use nlp100rs::chap05::neko;
use nlp100rs::Chunk;
use itertools::Itertools;
use std::ops::Add;

fn find_cross(x: &Vec<Chunk>, y: &Vec<Chunk>) -> Option<Chunk> {
    for xc in x {
        for yc in y {
            if xc == yc {
                return Some(xc.clone());
            }
        }
    }
    return None;
}

fn join(c: &Chunk, x: &Chunk, y: &Chunk) -> String {
    let mut result = String::new();
    for m in &c.morphs {
        match m.pos.as_str() {
            "記号" => continue,
            "名詞" if c == x => {
                result = result.add("X");
            }
            "名詞" if c == y => {
                result = result.add("Y");
            }
            _ => {
                result = result.add(&m.surface.clone());
            }
        }
    }
    result
}


/// 49. 名詞間の係り受けパスの抽出
/// 文中のすべての名詞句のペアを結ぶ最短係り受けパスを抽出せよ．ただし，
/// 名詞句ペアの文節番号がiiとjj（i<ji<j）のとき，係り受けパスは以下の
/// 仕様を満たすものとする．
///
/// * 問題48と同様に，パスは開始文節から終了文節に至るまでの各文節の表
///   現（表層形の形態素列）を"->"で連結して表現する
/// * 文節iiとjjに含まれる名詞句はそれぞれ，XとYに置換する
///
/// また，係り受けパスの形状は，以下の2通りが考えられる．
///
/// * 文節iiから構文木の根に至る経路上に文節jjが存在する場合: 文節iiか
///   ら文節jjのパスを表示
/// * 上記以外で，文節iiと文節jjから構文木の根に至る経路上で共通の文節
///   kkで交わる場合: 文節iiから文節kkに至る直前のパスと文節jjから文節
///   kkに至る直前までのパス，文節kkの内容を"|"で連結して表示
pub fn main() {
    let neko = neko();
    for s in neko {
        let ns: Vec<Chunk> = s.iter().filter(|c| c.has_pos("名詞")).cloned().collect();
        if ns.len() < 2 {
            continue;
        }

        let comb: Vec<(Chunk, Chunk)> = ns.iter().cloned().combinations().collect();
        for (from, until) in comb {
            if let Some(cs) = s.get_path_until(&from, &until) {
                let css: Vec<String> = cs.iter().map(|ch| join(ch, &from, &until)).collect();
                println!("{}", css.as_slice().join(" -> "));
            } else {
                let fs = s.get_path(&from);
                let us = s.get_path(&until);
                if let Some(cross) = find_cross(&fs, &us) {
                    let mut fs: Vec<String> = s.get_path_until(&from, &cross)
                        .unwrap()
                        .iter()
                        .map(|c| join(c, &from, &until))
                        .collect();
                    fs.pop().unwrap();
                    let mut us: Vec<String> = s.get_path_until(&until, &cross)
                        .unwrap()
                        .iter()
                        .map(|c| join(c, &from, &until))
                        .collect();
                    us.pop().unwrap();
                    println!("{} | {} | {}",
                             fs.as_slice().join(" -> "),
                             us.as_slice().join(" -> "),
                             cross.join());
                    // TODO untilの助詞を削除
                } else {
                    unreachable!("");
                }
            }
        }
    }
}
