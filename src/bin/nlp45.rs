extern crate nlp100rs;

use nlp100rs::chap05::neko;
use nlp100rs::Chunk;


/// 45. 動詞の格パターンの抽出
/// 今回用いている文章をコーパスと見なし，日本語の述語が取りうる格を調
/// 査したい． 動詞を述語，動詞に係っている文節の助詞を格と考え，述語
/// と格をタブ区切り形式で出力せよ．
///
/// ただし，出力は以下の仕様を満たすようにせよ．
///
/// * 動詞を含む文節において，最左の動詞の基本形を述語とする
/// * 述語に係る助詞を格とする
/// * 述語に係る助詞（文節）が複数あるときは，すべての助詞をスペース区
///   切りで辞書順に並べる
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
                println!("{}\t{}", p.base, cs.as_slice().join(" "));
            }
        }
    }
}
