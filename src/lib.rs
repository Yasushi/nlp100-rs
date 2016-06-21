extern crate flate2;
extern crate rustc_serialize;
extern crate regex;
extern crate itertools;

mod chap01;
mod chap02;
pub mod chap03;
pub mod chap04;

pub use chap04::Morph;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
