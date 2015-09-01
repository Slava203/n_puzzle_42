use algo::{Heuristic};
use algo::{AStarNode};
use npuzzle::{NPuzzle, Tile};
use std::collections::BinaryHeap;

pub struct AStar
{
	board:		NPuzzle,
	// closedset:	Vec<>,
	open_set:	BinaryHeap<AStarNode>,
}

impl AStar
{
	/// This is the a* algorithm
	fn determine_shortest_way(&mut self, start: Tile, end: Tile) {
		;
	}

	fn execute(&mut self) {
		// TODO add test for board being complete
		while !self.open_set.is_empty() {
		}
	}

	pub fn solve(initial_state: &NPuzzle, heu: &Heuristic) -> AStar
	{
		let mut to_return = AStar {
			board:		initial_state.clone(),
			open_set:	BinaryHeap::new(),
		};
		to_return.open_set.push(AStarNode::new_root(initial_state, heu));
		to_return.execute();
		to_return
	}
}
