use tools::tokenizer::{Token, Tokenizer, TokenInfo};
use npuzzle::tile::{Tile};
use tools::fn_str;
use npuzzle::{NPuzzle};

impl NPuzzle {
	fn split_one_line(line: &str, size: i32) -> Option<Vec<i32>> {
		let ints : Vec<i32> = line
				.split(' ')
				.map(|x| x.trim())
				.filter(|x| x.len() > 0)
				.map(|x| {fn_str::atoi(x).unwrap()})
				.collect();
		if ints.len() == size as usize {
			Some(ints)
		} else {
			None
		}
	}

	fn split_into_lines(to_parse: &String) -> Vec<&str> {
		to_parse.split('\n')
				.map(|x| x.trim())
				.filter(|x| x.len() > 0)
				.filter(|x| x.chars().next() != "#".chars().next())
				.collect::<Vec<&str>>()
	}

	// pub fn parse(size: i32, to_parse: &String)
	// 		-> Result<NPuzzle, &'static str> {

	// }

	/// Parse a string which describe the inital state of the npuzzle board.
	fn execute_parse(size: i32, lines: &Vec<&str>)
			-> Result<NPuzzle, &'static str> {
		let mut to_return = NPuzzle::new(size);
		if lines.len() != size as usize {
			return Err("Error parsing: wrong number of line");
		}

		// split lines into integer
		for line in lines {
			let ints = NPuzzle::split_one_line(line, size);
			if ints.is_some() {
				let mut tiles = ints.unwrap().iter()
						.map(|x| Tile::from_nbr(*x)).collect();
				to_return.append_tiles(&mut tiles);
			} else {
				return Err("Error parsing: one line is too short.");
			}
		}

		// test the validity of the parsed board
		let is_correct = to_return.is_correct();
		if is_correct.is_ok() {
			Ok(to_return)
		} else {
			Err(is_correct.err().unwrap())
		}
	}

	/// This function also parse the size of the NPuzzle.
	pub fn parse_with_size(to_parse: &String)
			-> Result<NPuzzle, &'static str> {
		let lines = NPuzzle::split_into_lines(to_parse);
		let size_err = fn_str::atoi::<i32>(lines[0]);
		if size_err.is_err() {
			return Err("Error parsing size into int");
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
