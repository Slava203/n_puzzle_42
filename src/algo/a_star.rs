use std::rc::Rc;
use std::fmt;
use algo::{AStarNode, HeuristicFn};
use npuzzle::{NPuzzle, Tile, Action};
use std::collections::BinaryHeap;

// BAN for boxed A* Node
type RcASN = Rc<AStarNode>;

pub struct AStar
{
	board:		NPuzzle,
	closed_set:	Vec<RcASN>,
	open_set:	BinaryHeap<RcASN>,
	result:		Option<RcASN>,
	nb_turn:	u32,
}

impl AStar
{
	/// Return true if the node has already been tested.
	fn is_in_closed_set(&self, node: &AStarNode) -> bool {
		for closed_state in self.closed_set.iter() {
			// println!("#cmp\n{:?}\n{:?}", closed_state.board(), node.board());
			if closed_state.board() == node.board() {
				// println!("is_in_closed_set");
				return true;
			}
		}
		return false;
	}

	/// Return a list of all the states which
	fn open_neighbours(&self, current: Rc<AStarNode>,
			game: &NPuzzle,
			heu: &HeuristicFn)
				-> Vec<RcASN> {
		// println!("open_neighbours current {:?} board {}", current, current.board());
		let mut to_return = Vec::new();
		for act in Action::list_all() {
			if current.board().action_allowed(act.clone()) {
				// println!("action_allowed {:?}", act);
				let new_node = Rc::new(
						AStarNode::new(act, Some(current.clone()), game, heu));
				// TODO test if new_node in open_set with lower cost
				// TODO jump only if is_in_closed_set && ttl_cost < new_node.ttl_cost ?
				if self.is_in_closed_set(&*new_node) {
					// println!("is in closed_set {}", new_node);
					continue ;
				}
				// println!("push {}", new_node);
				to_return.push(new_node);
			}
		}
		to_return
	}

	fn open_set_str(&self) -> String {
		let mut to_return = String::new();
		to_return = to_return + "[";
		for i in self.open_set.iter() {
			to_return = to_return + &format!("{} ", i)[..];
		}
		to_return = to_return + "]";
		to_return
	}

	fn execute(&mut self, game: &NPuzzle, heu: &HeuristicFn)
			-> Option<RcASN> {
		let root = Rc::new(AStarNode::new_root(game, heu));
		self.open_set.push(root);
		while !self.open_set.is_empty() {
			self.nb_turn += 1;
			println!("\n###nb_turn {:?}", self.nb_turn);
			let current = self.open_set.pop().unwrap();
			println!("current {} open_set {} board :\n{}",
					current, self.open_set_str(), current.board());

			// victory condition
			if game.is_complete(current.board()) {
				return Some(current);
			}

			let mut next = self.open_neighbours(current.clone(), game, heu);
			// println!("open_neighbours {:?}", next);
			// self.open_set.clear(); // TO DELETE
			self.closed_set.push(current);
			self.open_set.extend(next);
			// println!("new open_set {:?}", self.open_set_str());
			if self.nb_turn > 50 {break ;}
		}
		None
	}

	pub fn solve(game: &NPuzzle, heu: &HeuristicFn) -> AStar
	{
		let mut to_return = AStar {
			board:		game.clone(),
			open_set:	BinaryHeap::new(),
			closed_set:	Vec::new(),
			result:		None,
			nb_turn:	0,
		};
		to_return.result = to_return.execute(game, heu);
		to_return
	}

	pub fn get_result(&self) -> &Option<RcASN> {
		&self.result
	}

	pub fn print_result(&self) {
		println!("Result calculated in {} turn", self.nb_turn);
		match self.result {
			Some(ref x) => {
				let move_list = x.move_list();
				println!("{:?}", move_list);
				print!("{}", *x.board());
			},
			None		=> println!("The puzzle is unsolvable"),
		}
	}
}
