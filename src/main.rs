extern crate rand;

use npuzzle::{NPuzzle};

mod npuzzle;

fn main() {
	let party = NPuzzle::new_random(4);
    println!("{}", party);
}
