#[cfg(test)]
mod test
{
	use npuzzle::{NPuzzle, Action};
	use algo::{AStarNode};

	#[test]
	fn test() {
		let mut s = r#"
		3
		1 2 3
		4 0 5
		6 7 8
		"#;
		let np = NPuzzle::from_str(s);
		let root = AStarNode::new_root(game);
		let an1 = AStarNode::new_root(Action::Up, &np);
	}
}

