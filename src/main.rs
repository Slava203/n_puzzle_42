extern crate regex;
// extern crate num;
// extern crate core;
extern crate rand;

use npuzzle::{NPuzzle};

mod npuzzle;
mod tools;

fn main() {
	let board = NPuzzle::new_from_file("test");
	if board.is_ok() {
	    println!("{}", board.unwrap());
	} else {
		println!("{}", board.err().unwrap());
	}
}
