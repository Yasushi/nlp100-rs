extern crate nlp100rs;

use nlp100rs::chap05::neko;
use std::process::{Command, Stdio};
use std::io;
use std::io::Write;
use std::fs::File;
use std::env;
use std::str::FromStr;

/// 44. 係り受け木の可視化
/// 与えられた文の係り受け木を有向グラフとして可視化せよ．可視化には，
/// 係り受け木をDOT言語に変換し，Graphvizを用いるとよい．また，Python
/// から有向グラフを直接的に可視化するには，pydotを使うとよい．
pub fn main() {
    let no = env::args().skip(1).next().and_then(|s| usize::from_str(&s).ok()).unwrap_or(7);

    let mut dot = Command::new("dot")
        .arg("-Tpng")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    {
        let stdin = dot.stdin.as_mut().unwrap();
        write!(stdin, "digraph {{").unwrap();
        let neko = neko();
        for &(ref c1, ref c2) in &neko[no].pair() {
            write!(stdin, "\"{}\" -> \"{}\"", c1.join(), c2.join()).unwrap();
        }
        write!(stdin, "}}").unwrap();
        stdin.flush().unwrap();
    }

    let out = dot.wait_with_output()
        .unwrap_or_else(|e| panic!("failed to wait on child: {}", e));

    let mut stdout = out.stdout.as_slice();
    let mut f = File::create(format!("target/nlp44_{}.png", no)).unwrap();
    io::copy(&mut stdout, &mut f).unwrap();
}
