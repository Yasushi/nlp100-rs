extern crate nlp100rs;

use nlp100rs::chap05::neko;

/// 48. 名詞から根へのパスの抽出
/// 文中のすべての名詞を含む文節に対し，その文節から構文木の根に至るパ
/// スを抽出せよ． ただし，構文木上のパスは以下の仕様を満たすものとす
/// る．
///
/// * 各文節は（表層形の）形態素列で表現する
/// * パスの開始文節から終了文節に至るまで，各文節の表現を"->"で連結す
///   る
pub fn main() {
    let neko = neko();
    for s in neko {
        for c in s.iter() {
            if c.has_pos("名詞") {
                let cs: Vec<String> = s.get_path(&c)
                    .iter()
                    .map(|ch| ch.join())
                    .collect();
                println!("{}", cs.as_slice().join(" -> "));
            }
        }
    }
}
