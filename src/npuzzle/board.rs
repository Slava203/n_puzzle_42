use std::{ptr};
use std::str::FromStr;
use npuzzle::errors::{IncorrectBoardError, ParseError};
use std::fmt::{Formatter, Display, Error};
use npuzzle::{Tile, Action};
use npuzzle::parser;

#[derive(Debug, Clone)]
pub struct Board
{
	size:		usize,
	tiles:		Vec<Tile>,

	/// Coordinate x of the free tile
	x_free:		usize,

	/// Coordinate y of the free tile
	y_free:		usize,
}

impl Board
{
	pub fn new(size: usize) -> Board {
		Board {
			size:	size,
			tiles:	Vec::with_capacity(size * size),
			x_free:	0,
			y_free:	0,
		}
	}

	pub fn new_with_tiles(size: usize, tiles: Vec<Tile>) -> Board {
		let mut to_return = Board {
			size:	size,
			tiles:	tiles,
			x_free:		0,
			y_free:		0,
		};
		to_return.search_free_coord();
		to_return
	}

	pub fn append_tiles(&mut self, new_tiles: &mut Vec<Tile>) {
		self.tiles.extend(new_tiles.iter().cloned());
		self.search_free_coord();
	}

	fn search_free_coord(&mut self) {
		for (i, tile) in self.tiles.iter().enumerate() {
			if *tile == Tile::FREE {
				let (x, y) = self.xy_out_of_index(i);
				self.x_free = x;
				self.y_free = y;
				return ;
			}
		}
	}

	pub fn get_size(&self) -> usize {
		self.size
	}

	/// Return the number of tile in the npuzzle board including the empty tile.
	pub fn nb_tile(&self) -> usize {
		self.size * self.size
	}

	/// Get the tiles which coordinates are [x, y]
	pub fn get(&self, x: usize, y: usize) -> Tile {
		self.tiles[(y * self.size + x)].clone()
	}

	pub fn xy_out_of_index(&self, idx: usize) -> (usize, usize) {
		(idx % self.size, idx / self.size)
	}

	pub fn index_out_of_xy(&self, x: usize, y: usize) -> usize {
		y * self.size + x
	}

	pub fn action_allowed(&self, action: Action) -> bool {
		if self.x_free as i32 + action.impact_x() < 0 ||
				self.x_free as i32 + action.impact_x() >= self.size as i32 ||
				self.y_free as i32 + action.impact_y() < 0 ||
				self.y_free as i32 + action.impact_y() >= self.size as i32 {
			return false;
		}
		true
	}

	// change the board according to the action.
	pub fn execute_action(&mut self, action: Action) -> bool {
		if !self.action_allowed(action.clone()) {
			return false;
		}
		let x = (self.x_free as i32 + action.impact_x()) as usize;
		let y = (self.y_free as i32 + action.impact_y()) as usize;
		let idx_tile = self.index_out_of_xy(x, y);
		let idx_free = self.index_out_of_xy(self.x_free, self.y_free);
		self.tiles.swap(idx_tile, idx_free);
		self.x_free = x;
		self.y_free = y;
		true
	}

	// Return true if there is no error in this board
	pub fn is_correct(&self) -> Result<(), IncorrectBoardError> {
		// test number of tile
		if self.tiles.len() != self.nb_tile() as usize {
			return Err(IncorrectBoardError::WrongNumberOfTile{
				found:		self.tiles.len(),
				expected:	self.nb_tile(),
			});
		}

		// test if the tiles are the one expected
		let mut used_numbers : Vec<(i32, bool)> = (0..self.nb_tile())
				.map(|x| (x as i32, false)).collect();
		for i in (0..self.nb_tile()) {
			let tile_nbr = self.tiles[i as usize].to_nbr();
			if tile_nbr as usize > self.nb_tile() - 1 {
				return Err(IncorrectBoardError::OutOfBoundTile{tile: tile_nbr});
			}
			let (_, already_in) = used_numbers[tile_nbr as usize];
			if already_in {
				return Err(IncorrectBoardError::DuplicatedTile{tile: tile_nbr});
			}
			used_numbers[tile_nbr as usize] = (tile_nbr, true);
		}

		//every thing is ok !
		Ok(())
	}

	/// Return true it the tiles of this board are align as expected.
	pub fn is_complete(&self) -> bool {
		for i in (1..self.nb_tile()) {
			if self.tiles[i - 1].to_nbr() != i as i32 {
				return false;
			}
		}
		true
	}

	pub fn get_tiles(&self) -> &Vec<Tile> {
		&self.tiles
	}
}

impl Display for Board
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

impl PartialEq for Board {
    fn eq(&self, other: &Board) -> bool {
    	if self.size != other.size {
    		return false;
    	}
    	for i in (0..self.nb_tile()) {
    		if self.tiles[i] != other.tiles[i] {
    			return false;
    		}
    	}
    	true
    }

    fn ne(&self, other: &Board) -> bool {
    	!self.eq(other)
    }
}
