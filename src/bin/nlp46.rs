extern crate nlp100rs;

use nlp100rs::chap05::neko;
use nlp100rs::Chunk;

/// 46. 動詞の格フレーム情報の抽出
/// 45のプログラムを改変し，述語と格パターンに続けて項（述語に係ってい
/// る文節そのもの）をタブ区切り形式で出力せよ．45の仕様に加えて，以下
/// の仕様を満たすようにせよ．
///
/// * 項は述語に係っている文節の単語列とする（末尾の助詞を取り除く必要
///   はない）
/// * 述語に係る文節が複数あるときは，助詞と同一の基準・順序でスペース
///   区切りで並べる
pub fn main() {
    let neko = neko();
    for s in neko {
        let ps: Vec<&Chunk> = s.iter().filter(|c| c.has_pos("動詞")).collect();
        for c in ps.iter() {
            if let Some(p) = c.get_predicate() {
                let srcs = s.get_chunks(&c.srcs);
                let cs: Vec<String> = srcs.iter()
                    .flat_map(|c| c.morphs.iter().filter(|m| m.pos == "助詞").map(|m| m.base.clone()))
                    .collect();
                let args: Vec<String> = srcs.iter().map(|c| c.join()).collect();
                println!("{}\t{}\t{}",
                         p.base,
                         cs.as_slice().join(" "),
                         args.as_slice().join(" "));
            }
        }
    }
}
