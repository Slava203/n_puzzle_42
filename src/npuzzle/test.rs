#[cfg(test)]
mod test
{
	use super::*;
	use npuzzle::{NPuzzle, Board, Action};

	fn test_one_is_complete(s: &str, is_complete: bool) {
		let np = NPuzzle::new_from_str(&s.to_string()).unwrap();
		assert!(np.get_initial_state().is_complete() == is_complete);
	}

	#[test]
	fn test_is_complete() {
		let mut s = r#"# This puzzle is solvable
	3
	4 5 8
	2 0 7
	1 3 6"#;
		test_one_is_complete(s, false);
		s = r#"3
	1 2 3
	4 5 6
	7 8 0"#;
		test_one_is_complete(s, true);
		s = r#"3
	1 2 3
	4 5 6
	8 7 0"#;
		test_one_is_complete(s, false);
		s = r#"5
	18 21  9 24 16
	12 10 20 22  2
	14 19 17  8  5
	 3  6 11 13  7
	 4  1 15 23  0"#;
		test_one_is_complete(s, false);
		s = r#"	5
	1   2  3  4  5
	 6  7  8  9 10
	11 12 13 14 15
	16 17 18 19 20
	21 22 23 24  0"#;
		test_one_is_complete(s, true);
	}

	fn one_test_board_execute_action(b1s: &str, b2s: &str, act: Action) {
		let mut begin = Board::parse_with_size(&b1s.to_string()).unwrap();
		let expected = Board::parse_with_size(&b2s.to_string()).unwrap();
		begin.execute_action(act);
		println!("######\n{} \n=>\n {}\n", begin, expected);
		assert!(begin == expected);
	}

	#[test]
	fn test_board_execute_action() {
		let mut s2 = "";
		let mut s = r#"3
	1 2 3
	4 5 6
	8 7 0"#;
		one_test_board_execute_action(s, s, Action::Right);
		s = r#"3
	1 2 3
	4 5 6
	8 7 0"#;
		s2 = r#"3
	1 2 3
	4 5 6
	8 0 7"#;
		one_test_board_execute_action(s, s2, Action::Left);
		s = r#"3
	1 2 3
	4 5 6
	8 0 7"#;
		s2 = r#"3
	1 2 3
	4 0 6
	8 5 7"#;
		one_test_board_execute_action(s, s2, Action::Up);
		s = r#"3
	1 3 2
	4 5 0
	7 8 6 "#;
		s2 = r#"3
	1 3 0
	4 5 2
	7 8 6 "#;
		one_test_board_execute_action(s, s2, Action::Up);
	}
}

