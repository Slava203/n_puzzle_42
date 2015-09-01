use rand::Rng;
use rand;
use std::fmt::{Formatter, Display, Error};
use npuzzle::tile::{Tile};
use npuzzle::errors::{IncorrectBoardError, ParseError};
use std::path::Path;
use std::error;
use std::fs::File;
use std::io::prelude::*;
use npuzzle::{Board};
use std::io;

/// This structure represent a NPuzzle game instance.
#[derive(Debug, Clone)]
pub struct NPuzzle
{
	size:			usize,
	initial_state:	Board,
	goal_state:		Board,
}

impl NPuzzle
{
	fn new(size: usize, initial_state: Board) -> NPuzzle {
		NPuzzle{
			size:			size,
			initial_state:	initial_state,
			goal_state:		NPuzzle::create_goal_state(size),
		}
	}

	fn create_goal_state(size: usize) -> Board {
		let mut to_return = Vec::with_capacity(size * size);
		for i in (0..(size * size - 1)) {
			to_return.push(Tile::from_nbr((i + 1) as i32));
		}
		to_return.push(Tile::from_nbr(0));
		Board::new_with_tiles(size, to_return)
	}

	/// Return a random number which is not declared as used in
	/// the used_numbers vector.
	fn random_tile(used_numbers: &mut Vec<(i32, bool)>) -> i32 {
		let mut rng = rand::thread_rng();
		let rand = rng.gen::<usize>() % used_numbers.len();
		for i in (0..used_numbers.len()) {
			let index = (rand + i) % used_numbers.len();
			let &mut num = &mut used_numbers[index];
			if !num.1 {
				used_numbers[index] = (num.0, true);
				return num.0;
			}
		}
		panic!("Error NPuzzle::random_tile, no unused tile.")
	}

	/// Return a NPuzzle which values as been set randomly
	pub fn new_random(size: usize) -> NPuzzle {
		let mut board = Board::new(size);

		// This array contain all of the number to put in the board, and
		// associate a boolean to say if the corresponding number is already
		// in the board
		let mut used_numbers = (0..board.nb_tile())
				.map(|x| (x as i32, false)).collect();

		let mut new_tiles = Vec::with_capacity(board.nb_tile());
		for _ in (0..board.nb_tile()) {
			let nbr = NPuzzle::random_tile(&mut used_numbers);
			new_tiles.push(Tile::from_nbr(nbr));
		}
		board.append_tiles(&mut new_tiles);
		NPuzzle::new(size, board)
	}

	pub fn board_res_into_npuzzle(np_res: Result<Board, ParseError>)
			-> Result<NPuzzle, ParseError> {
		if np_res.is_err() {
			Err(np_res.err().unwrap())
		} else {
			let board = np_res.unwrap();
			Ok(NPuzzle::new(board.get_size(), board))
		}
	}

	pub fn new_from_stdin()
			-> Result<NPuzzle, ParseError> {
		let mut reader = io::stdin();
		let mut s = String::new();
		match reader.read_to_string(&mut s) {
			Err(why)	=> panic!("couldn't read stdin {}",
								error::Error::description(&why)),
			Ok(_)		=> print!(""),
		};
		NPuzzle::board_res_into_npuzzle(NPuzzle::parse_with_size(&s))
	}

	pub fn new_from_str(s: &String)
			-> Result<NPuzzle, ParseError> {
		print!("{:?}", NPuzzle::parse_with_size(&s));
		print!("{:?}", NPuzzle::board_res_into_npuzzle(NPuzzle::parse_with_size(&s)));
		NPuzzle::board_res_into_npuzzle(NPuzzle::parse_with_size(&s))
	}

	pub fn new_from_file(file_name: &str)
			-> Result<NPuzzle, ParseError> {
		let path = Path::new(&file_name);
		let display = path.display();

		let mut file = match File::open(&path) {
			Err(why)	=> panic!("couldn't open {}: {}", display,
								error::Error::description(&why)),
			Ok(file)	=> file,
		};

		// Read the file contents into a string, returns `io::Result<usize>`
		let mut s = String::new();
		match file.read_to_string(&mut s) {
			Err(why)	=> panic!("couldn't read {}: {}",
								display,
								error::Error::description(&why)),
			Ok(_)		=> print!(""),
		};
		NPuzzle::board_res_into_npuzzle(NPuzzle::parse_with_size(&mut s))
	}

	/// Return the number of tile in the npuzzle board including the empty tile.
	pub fn nb_tile(&self) -> usize {
		self.size * self.size
	}

	pub fn get_size(&self) -> usize {
		self.size
	}

	pub fn get_initial_state(&self) -> &Board {
		&self.initial_state
	}

	pub fn get_goal_state(&self) -> &Board {
		&self.goal_state
	}
}

impl Display for NPuzzle
{
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error>
	{
		write!(f, "{}", self.initial_state);
		Ok(())
	}
}
