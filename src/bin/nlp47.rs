extern crate nlp100rs;

use nlp100rs::chap05::neko;
use nlp100rs::{Chunk, Morph};


/// 47. 機能動詞構文のマイニング
/// 動詞のヲ格にサ変接続名詞が入っている場合のみに着目したい．46のプロ
/// グラムを以下の仕様を満たすように改変せよ．
///
/// * 「サ変接続名詞+を（助詞）」で構成される文節が動詞に係る場合のみ
///   を対象とする
/// * 述語は「サ変接続名詞+を+動詞の基本形」とし，文節中に複数の動詞が
///   あるときは，最左の動詞を用いる
/// * 述語に係る助詞（文節）が複数あるときは，すべての助詞をスペース区
///   切りで辞書順に並べる
/// * 述語に係る文節が複数ある場合は，すべての項をスペース区切りで並べ
///   る（助詞の並び順と揃えよ）
pub fn main() {
    let neko = neko();
    for s in neko {
        let ps: Vec<&Chunk> = s.iter().filter(|c| c.has_pos("動詞")).collect();
        for c in ps.iter() {
            if let Some(p) = c.get_predicate() {
                let srcs: Vec<Chunk> = s.get_chunks(&c.srcs);
                let fs = srcs.iter()
                    .filter(|c| {
                        c.morphs.len() == 2 && c.morphs[0].pos1 == "サ変接続" && c.morphs[1].surface == "を"
                    })
                    .next();

                if let Some(f) = fs {

                    let mut ss: Vec<usize> = f.srcs
                        .iter()
                        .chain(c.srcs.iter())
                        .cloned()
                        .filter(|&u| u != f.no)
                        .collect();
                    ss.sort();

                    let cs = s.get_chunks(&ss);
                    let ps: Vec<Morph> = cs.iter()
                        .flat_map(|c| c.get_by_pos("助詞").last().cloned())
                        .collect();

                    let pss: Vec<String> = ps.iter().map(|m| m.surface.clone()).collect();
                    let css: Vec<String> = cs.iter().map(|c| c.join()).collect();

                    println!("{}{}\t{}\t{}",
                             f.join(),
                             p.base,
                             pss.as_slice().join(" "),
                             css.as_slice().join(" "));

                }
            }
        }
    }
}
