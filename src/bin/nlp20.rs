extern crate nlp100rs;

use nlp100rs::chap03;

pub fn main() {
    let r = chap03::get_country_text("イギリス").unwrap();
    println!("{}", r);
}
