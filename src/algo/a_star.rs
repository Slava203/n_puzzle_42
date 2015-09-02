use std::rc::Rc;
use algo::{AStarNode, Heuristic};
use npuzzle::{NPuzzle, Tile, Action};
use std::collections::BinaryHeap;

// BAN for boxed A* Node
type RcASN = Rc<AStarNode>;

pub struct AStar
{
	board:		NPuzzle,
	closed_set:	Vec<RcASN>,
	open_set:	BinaryHeap<RcASN>,
}

impl AStar
{
	/// Return true if the node has already been tested.
	fn is_in_closed_set(&self, node: &AStarNode) -> bool {
		for closed_state in self.closed_set.iter() {
			if closed_state.board() == node.board() {
				return false;
			}
		}
		return true;
	}

	fn open_neighbours(&self, current: Rc<AStarNode>, game: &NPuzzle, heu: &Heuristic)
			-> Vec<RcASN> {
		let mut to_return = Vec::new();
		for act in Action::list_all() {
			if current.board().action_allowed(act.clone()) {
				let new_node = Rc::new(
						AStarNode::new(act, Some(current.clone()), game, heu));
				// TODO test if new_node in open_set with lower cost
				if self.is_in_closed_set(&*new_node) {
					continue ;
				}
				to_return.push(new_node);
			}
		}
		to_return
	}

	fn execute(&mut self, game: &NPuzzle, heu: &Heuristic)
			-> Option<RcASN> {
		let root = Rc::new(AStarNode::new_root(game, heu));
		self.open_set.push(root);
		// TODO add test for board being complete
		while !self.open_set.is_empty() {
			let current = self.open_set.pop().unwrap();

			// victory condition
			if current.is_complete() {
				return Some(current);
			}

			let mut next = self.open_neighbours(current, game, heu);
			self.open_set.extend(next);
		}
		None
	}

	pub fn solve(game: &NPuzzle, heu: &Heuristic) -> AStar
	{
		let mut to_return = AStar {
			board:		game.clone(),
			open_set:	BinaryHeap::new(),
			closed_set:	Vec::new(),
		};
		to_return.execute(game, heu);
		to_return
	}
}
