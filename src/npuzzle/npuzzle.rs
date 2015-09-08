use std::str::FromStr;
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
		let mut puzzle = Vec::with_capacity(size * size);

		//create a list of -1
		for i in (0..(size * size)) {
			puzzle.push(-1);
		}

		//convert it to snail
		println!("###for {:?}", size);
		let mut x = 0i32;
		let mut ix = 1i32;
		let mut y = 0i32;
		let mut iy = 0i32;
		let mut cur = 1i32;
		let s = size as i32;
		loop {
			puzzle[(x + y * s) as usize] = cur;
			println!("{:?}", x + y * s);
			if cur == 0 {
				break ;
			}
			cur += 1;
			if x + ix == s as i32 ||
					x + ix < 0 ||
					(ix != 0 && puzzle[(x + ix + y * s) as usize] != -1) {
				iy = ix;
				ix = 0;
			}
			else if x + (y+iy) < (s * s + 1) &&
					(y + iy == s as i32 ||
					 					y + iy < 0 ||
					 					(iy != 0 && puzzle[(x + (y+iy) * s) as usize] != -1)) {
				ix = -iy;
				iy = 0;
			}
			x += ix;
			y += iy;
			if cur == s * s {
				cur = 0;
			}
		}
		Board::new_with_tiles(size, puzzle.iter().map(|x| Tile::from_nbr(*x)).collect())
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
		NPuzzle::board_res_into_npuzzle(Board::parse_with_size(&s))
	}

	pub fn new_from_str(s: &String)
			-> Result<NPuzzle, ParseError> {
		NPuzzle::board_res_into_npuzzle(Board::parse_with_size(&s))
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
		NPuzzle::board_res_into_npuzzle(Board::parse_with_size(&mut s))
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

	///Return true if the board is complete
	pub fn is_complete(&self, board: &Board) -> bool {
		if board.get_size() != self.size {
			return false;
		}
		let board_tiles = board.get_tiles();
		let goal_tiles = self.goal_state.get_tiles();
		for i in (0..self.nb_tile()) {
			if goal_tiles[i] != board_tiles[i] {
				return false;
			}
		}
		return true;
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
