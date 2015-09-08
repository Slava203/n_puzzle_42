#[cfg(test)]
mod test
{
	use super::*;
	use npuzzle::{NPuzzle, Board, Action};

	fn test_one_is_complete(s: &str, is_complete: bool) {
		let np = NPuzzle::new_from_str(&s.to_string()).unwrap();
		let str_is_complete = np.is_complete(np.get_initial_state());
		assert!(str_is_complete == is_complete);
	}

	#[test]
	fn test_is_complete() {
		let mut s = r#"# This puzzle is solvable
	3
	1 3 2
	8 0 4
	7 6 5"#;
		test_one_is_complete(s, false);
		s = r#"3
	1 2 3
	8 0 4
	7 6 5"#;
		test_one_is_complete(s, true);
		s = r#"3
	1 2 3
	8 5 4
	7 6 0"#;
		test_one_is_complete(s, false);
		s = r#"5
	 1  2  3  4  5
	16 17 18 19  6
	15 24 21 20  7
	14 23 22  0  8
	13 12 11 10  9"#;
		test_one_is_complete(s, false);
		s = r#"	5
	 1  2  3  4  5
	16 17 18 19  6
	15 24  0 20  7
	14 23 22 21  8
	13 12 11 10  9"#;
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

	fn one_test_create_goal_state(i: usize, s: &str) {
		let np = NPuzzle::new_random(i);
		let board = Board::parse_with_size(&s.to_string()).unwrap();
		println!("{:?}", *np.get_goal_state());
		assert!(*np.get_goal_state() == board);
	}

	#[test]
	fn test_create_goal_state() {
		let mut s = r#"3
	1 2 3
	8 0 4
	7 6 5 "#;
		one_test_create_goal_state(3, s);
		s = r#"4
	 1  2  3  4
	12 13 14  5
	11  0 15  6
	10  9  8  7 "#;
		one_test_create_goal_state(4, s);
		s = r#"5
	 1  2  3  4  5
	16 17 18 19  6
	15 24  0 20  7
	14 23 22 21  8
	13 12 11 10  9"#;
		one_test_create_goal_state(5, s);
	}
}

