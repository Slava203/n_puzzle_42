use std::error;
use std::fmt::{Formatter, Display, Error};

#[derive(Debug)]
pub enum IncorrectBoardError {
	WrongNumberOfTile{found: usize, expected: usize},
	OutOfBoundTile{tile: i32},
	DuplicatedTile{tile: i32},
}

impl Display for IncorrectBoardError
{
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error>
	{
		write!(f, "n puzzle board incorrect: ");
		match *self {
			IncorrectBoardError::WrongNumberOfTile{
					found: found, expected: expected
			}	=>
				write!(f, "wrong number of tile, expected {}, found {}",
						expected, found),
			IncorrectBoardError::OutOfBoundTile{tile: tile}		=>
				write!(f, "tile out of bound {}", tile),
			IncorrectBoardError::DuplicatedTile{tile: tile}		=>
				write!(f, "duplicated tile {}", tile),
		};
		Ok(())
	}
}

impl error::Error for IncorrectBoardError {
	fn description(&self) -> &str {
		match *self {
			IncorrectBoardError::WrongNumberOfTile{found :_, expected: _}
					=> "wrong number of tile",
			IncorrectBoardError::OutOfBoundTile{tile: _}
					=> "tile out of bound",
			IncorrectBoardError::DuplicatedTile{tile: _}
					=> "duplicated tile",
		}
	}

	fn cause(&self) -> Option<&error::Error> {
		None
	}
}

#[derive(Debug)]
pub enum ParseError {
	ParseSize,
	IncorrectBoard(IncorrectBoardError),
}

impl Display for ParseError
{
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error>
	{
		write!(f, "Parse error: ");
		match *self {
			ParseError::ParseSize	=> write!(f, "cannot parse size"),
			ParseError::IncorrectBoard(ref err)	=> write!(f, "{}", err),
		};
		Ok(())
	}
}

impl error::Error for ParseError {
	fn description(&self) -> &str {
		match *self {
			ParseError::ParseSize				=> "cannot parse size",
			ParseError::IncorrectBoard(ref err)	=> err.description(),
		}
	}

	fn cause(&self) -> Option<&error::Error> {
		match *self {
			ParseError::IncorrectBoard(ref err) => Some(err),
			_									=> None,
		}
	}
}
