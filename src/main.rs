extern crate term_painter;
extern crate regex;
extern crate getopts;
extern crate rand;

mod algo;
mod npuzzle;
mod tools;
mod options;

use npuzzle::{NPuzzle};
use options::{Options, Input};
use algo::{AStar, HeuristicFn};
use algo::heuristic;

fn np_from_option(opts: &Options) -> NPuzzle {
	match opts.input {
		Input::Stdin =>			{
			let np_res = NPuzzle::new_from_stdin();
			if np_res.is_err() {
				println!("{}", np_res.err().unwrap());
				std::process::exit(0)
			}
			np_res.unwrap()
		},
		Input::Random(size) =>	{
			NPuzzle::new_random(size)
		},
		Input::File(ref file_name) =>	{
			let np_res = NPuzzle::new_from_file(&file_name[..]);
			if np_res.is_err() {
				println!("{}", np_res.err().unwrap());
				std::process::exit(0)
			}
			np_res.unwrap()
		},
	}
}

fn main() {
	let options = Options::new();
	if options.check_options() {
		return ;
	}
	let np = np_from_option(&options);
	let heuri = heuristic::from_option(options.heuristic);
	let astar = AStar::solve(&np, &heuri);
    // println!("{}", np);
    astar.print_result();
}
