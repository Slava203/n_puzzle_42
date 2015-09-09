#[cfg(test)]
mod test
{
	use npuzzle::{NPuzzle, Board, Action};
	use algo::{AStarNode};
	use algo::heuristic;

	fn one_manh_test(initial: &str, expected: i32) {
		let np = NPuzzle::new_from_str(&initial.to_string()).unwrap();
		let nbr = heuristic::manhattan(np.get_initial_state(), &np);
		assert!(nbr == expected);
	}

	#[test]
	fn test_manhattan() {
		let mut s = r#"3
			1 2 3
			8 0 4
			7 6 5"#;
		one_manh_test(s, 0);
		let mut s = r#"3
			1 2 3
			8 4 0
			7 6 5"#;
		one_manh_test(s, 2);
		let mut s = r#"3
			1 2 3
			8 4 5
			7 0 6"#;
		one_manh_test(s, 4);
		let mut s = r#"3
			5 2 3
			8 0 4
			7 6 1"#;
		one_manh_test(s, 8);
	}

	fn one_misplaced_tiles_test(initial: &str, expected: i32) {
		let np = NPuzzle::new_from_str(&initial.to_string()).unwrap();
		let nbr = heuristic::misplaced_tiles(np.get_initial_state(), &np);
		assert!(nbr == expected);
	}

	#[test]
	fn test_misplaced_tiles() {
		let mut s = r#"3
			1 2 3
			8 0 4
			7 6 5"#;
		one_misplaced_tiles_test(s, 0);
		let mut s = r#"3
			8 0 4
			1 2 3
			7 6 5"#;
		one_misplaced_tiles_test(s, 6);
		let mut s = r#"3
			1 2 3
			8 4 0
			7 6 5"#;
		one_misplaced_tiles_test(s, 2);
	}
}

