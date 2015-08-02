extern crate weirustqi;

use weirustqi::base::board::*;

//use weirustqi::base::board;

pub fn main() {
    println!("Hello wei-rust-qi");
    let b = Board::new(19);
    println!("{}", b);
}
