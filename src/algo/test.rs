#[cfg(test)]
mod test
{
	use npuzzle::{NPuzzle, Board, Action};
	use algo::{AStarNode};
	use algo::heuristic;

	fn one_manh_test(initial: &str, expected: i32) {
		let np = NPuzzle::new_from_str(&initial.to_string()).unwrap();
		let nbr = heuristic::manhattan(np.get_initial_state(),
				np.get_goal_state());
		assert!(nbr == expected);
	}

	#[test]
	fn test_manhattan() {
		let mut s = r#"3
		1 2 3
		4 5 6
		7 8 0"#;
		one_manh_test(s, 0);
		let mut s = r#"3
		1 3 2
		4 5 6
		7 8 0"#;
		one_manh_test(s, 2);
		let mut s = r#"3
		1 2 7
		4 5 6
		3 8 0"#;
		one_manh_test(s, 8);
		let mut s = r#"3
		1 3 6
		4 5 2
		7 8 0"#;
		one_manh_test(s, 4);
	}
}

