use std::fmt::{Formatter, Display, Error};

#[derive(Debug, Clone, PartialEq)]
pub enum Tile {
    NUMBER(i32),
    FREE,
}

impl Tile {
	pub fn from_nbr(nbr: i32) -> Tile {
		if nbr < 0 {
			panic!("Error no tile is inferior to 0.");
		}
		if nbr == 0 {
			Tile::FREE
		}else{
			Tile::NUMBER(nbr)
		}
	}

	pub fn to_nbr(&self) -> i32 {
		match *self {
			Tile::FREE		=> 0,
			Tile::NUMBER(x)	=> x,
		}
	}
}

impl Display for Tile
{
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error>
	{
		let mut to_return = Ok(());
		match self {
			&Tile::NUMBER(x)	=> to_return =
									to_return.and(write!(f, "{:<4}", x)),
			&Tile::FREE			=> to_return =
									to_return.and(write!(f, "X   ")),
		}
		to_return
	}
}
