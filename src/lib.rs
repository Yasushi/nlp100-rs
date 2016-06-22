extern crate flate2;
extern crate rustc_serialize;
extern crate regex;
extern crate itertools;

mod chap01;
mod chap02;
pub mod chap03;
pub mod chap04;
mod chap05;

pub use chap04::Morph;
pub use chap05::Chunk;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
