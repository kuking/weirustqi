extern crate weirustqi;

use weirustqi::base::board::*;
use weirustqi::base::coord::*;

use std::str::FromStr;

//use weirustqi::base::board;

pub fn main() {
    if cfg!(debug_assertions) {
        println!("debug build")
    } else {
        println!("release build")
    }


    println!("Hello wei-rust-qi");
    let b = Board::new(19);
    println!("{}", b);

    println!("J18 is {} after Coord::from_str", Coord::from_str(&"j18").unwrap());

}
