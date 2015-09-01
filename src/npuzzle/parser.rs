use tools::fn_str;
use npuzzle::errors::{IncorrectBoardError, ParseError};
use npuzzle::{NPuzzle, Tile, Board};

impl NPuzzle {
	fn split_one_line(line: &str, size: usize) -> Vec<i32> {
		line.split(' ')
				.map(|x| x.trim())
				.filter(|x| x.len() > 0)
				.map(|x| {fn_str::atoi(x).unwrap()})
				.collect()
	}

	fn split_into_lines(to_parse: &String) -> Vec<&str> {
		to_parse.split('\n')
				.map(|x| x.trim())
				.filter(|x| x.len() > 0)
				.filter(|x| x.chars().next() != "#".chars().next())
				.collect::<Vec<&str>>()
	}

	/// Parse a string which describe the inital state of the npuzzle board.
	fn execute_parse(size: usize, lines: &Vec<&str>)
			-> Result<Board, ParseError> {
		let mut board = Board::new(size);

		// split lines into integer
		for line in lines {
			let ints : Vec<i32> = NPuzzle::split_one_line(line, size);
			let mut tiles = ints.iter().map(|x| Tile::from_nbr(*x)).collect();
			board.append_tiles(&mut tiles);
		}

		// test the validity of the parsed board
		let is_correct = board.is_correct();
		if is_correct.is_ok() {
			Ok(board)
		} else {
			Err(ParseError::IncorrectBoard(is_correct.err().unwrap()))
		}
	}

	/// This function also parse the size of the Board.
	pub fn parse_with_size(to_parse: &String)
			-> Result<Board, ParseError> {
		let lines = NPuzzle::split_into_lines(to_parse);
		let size_err = fn_str::atoi::<usize>(lines[0]);
		if size_err.is_err() {
			return Err(ParseError::ParseSize);
		}
		let lines_reduce = (&lines[1..]).to_vec();
		NPuzzle::execute_parse(size_err.unwrap(), &lines_reduce)
	}
}

#[cfg(test)]
mod test
{
	use super::*;
	use npuzzle::npuzzle::{NPuzzle};
	use npuzzle::tile::{Tile};

	#[test]
	fn test_parse1() {
		let mut str1 = r#"2
		1 2
		3 0
		"#;
		println!("str {:?}", str1);
		let np = NPuzzle::parse_with_size(&mut str1.to_string()).unwrap();
		// println!("size {:?}", np.get_size());
		assert!(np.get_size() == 2);
		assert!(np.get(0, 0) == Tile::NUMBER(1));
		assert!(np.get(1, 0) == Tile::NUMBER(2));
		assert!(np.get(0, 1) == Tile::NUMBER(3));
		assert!(np.get(1, 1) == Tile::FREE);
	}

	/// test for size
	#[test]
	#[should_panic]
	fn test_parse2() {
		let mut str1 = r#"3
		1 2
		3 0
		"#;
		println!("str {:?}", str1);
		let np = NPuzzle::parse_with_size(&mut str1.to_string()).unwrap();
	}

	#[test]
	#[should_panic]
	fn test_parse3() {
		let mut str1 = r#"3
		1 1
		3 0
		"#;
		println!("str {:?}", str1);
		let np = NPuzzle::parse_with_size(&mut str1.to_string()).unwrap();
	}

	#[test]
	#[should_panic]
	fn test_parse4() {
		let mut str1 = r#"3
		1
		3 0
		"#;
		println!("str {:?}", str1);
		let np = NPuzzle::parse_with_size(&mut str1.to_string()).unwrap();
	}
}
