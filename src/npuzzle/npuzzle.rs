use rand::Rng;
use rand;
use std::fmt::{Formatter, Display, Error};
use npuzzle::tile::{Tile};
use std::path::Path;
use std::error;
use std::fs::File;
use std::io::prelude::*;

/// This structure represent a NPuzzle game instance.
#[derive(Debug)]
pub struct NPuzzle
{
	size:	usize,
	tiles:	Vec<Tile>,
}

impl NPuzzle
{
	pub fn new(size: usize) -> NPuzzle {
		NPuzzle{
			size:	size,
			tiles:	Vec:: with_capacity((size * size)),
		}
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
				// num.1 = true;
				used_numbers[index] = (num.0, true);
				return num.0;
			}
		}
		panic!("Error NPuzzle::random_tile, no unused tile.")
	}

	/// Return a NPuzzle which values as been set randomly
	pub fn new_random(size: usize) -> NPuzzle {
		let mut to_return = NPuzzle::new(size);

		// This array contain all of the number to put in the board, and
		// associate a boolean to say if the corresponding number is already
		// in the board
		let mut used_numbers = (0..to_return.nb_tile())
				.map(|x| (x as i32, false)).collect();

		for _ in (0..to_return.nb_tile()) {
			let nbr = NPuzzle::random_tile(&mut used_numbers);
			to_return.tiles.push(Tile::from_nbr(nbr));
		}
		to_return
	}

	pub fn new_from_file(file_name: &str)
			-> Result<NPuzzle, &'static str> {
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
		NPuzzle::parse_with_size(&mut s)
	}

	pub fn append_tiles(&mut self, new_tiles: &mut Vec<Tile>) {
		self.tiles.extend(new_tiles.iter().cloned());
	}

	/// Return the number of tile in the npuzzle board including the empty tile.
	pub fn nb_tile(&self) -> usize {
		self.size * self.size
	}

	/// Get the tiles which coordinates are [x, y]
	pub fn get(&self, x: usize, y: usize) -> Tile {
		self.tiles[(y * self.size + x)].clone()
	}

	pub fn get_size(&self) -> usize {
		self.size
	}

	pub fn is_correct(&self) -> Result<(), &'static str> {
		// test number of tile
		if self.tiles.len() != self.nb_tile() as usize {
			return Err("NPuzzle board incorrect : not the required number of tile");
		}

		// test if the tiles are the one expected
		let mut used_numbers : Vec<(i32, bool)> = (0..self.nb_tile())
				.map(|x| (x as i32, false)).collect();
		for i in (0..self.nb_tile()) {
			let tile_nbr = self.tiles[i as usize].to_nbr();
			if tile_nbr as usize > self.nb_tile() - 1 {
				return Err("NPuzzle board incorrect : tile number out of bound");
			}
			let (_, already_in) = used_numbers[tile_nbr as usize];
			if already_in {
				return Err("NPuzzle board incorrect : duplicated tile");
			}
			used_numbers[tile_nbr as usize] = (tile_nbr, true);
		}

		//every thing is ok !
		Ok(())
	}
}

impl Display for NPuzzle
{
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error>
	{
		let mut to_return = Ok(());
		to_return = to_return.and(write!(f, "size : {}\n", self.size));
		for y in (0..self.size) {
			for x in (0..self.size) {
				to_return = to_return.and(write!(f, "{:<4} ", self.get(x, y)));
			}
			to_return = to_return.and(write!(f, "\n"));
		}
		to_return
	}
}
