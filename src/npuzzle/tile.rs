use std::fmt::{Formatter, Display, Error};

#[derive(Debug, Clone)]
pub enum Tile {
    NUMBER(i32),
    FREE,
}

impl Tile {
	pub fn from_nbr(nbr: i32) -> Tile {
		if nbr == 0 {
			Tile::FREE
		}else{
			Tile::NUMBER(nbr)
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
