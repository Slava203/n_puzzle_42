extern crate regex;
extern crate getopts;
extern crate rand;


mod npuzzle;
mod tools;
mod options;

use npuzzle::{NPuzzle};
use options::Options;

fn main() {
	let options = Options::new();
	if options.check_options() {
		return ;
	}
	let board = NPuzzle::new_from_file("test");
	if board.is_ok() {
	    println!("{}", board.unwrap());
	} else {
		println!("{}", board.err().unwrap());
	}
}
