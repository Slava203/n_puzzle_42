use std::rc::Rc;
use algo::{AStarNode, HeuristicFn};
use npuzzle::{NPuzzle, Action};
use std::collections::{BinaryHeap, HashMap};

// BAN for boxed A* Node
type RcASN = Rc<AStarNode>;

pub struct AStar
{
	board:		NPuzzle,
	closed_set:	HashMap<RcASN, ()>,
	open_set:	BinaryHeap<RcASN>,
	result:		Option<RcASN>,
	nb_turn:	u32,
	nb_state:	u32,
}

impl AStar
{
	/// Return true if the node has already been tested.
	fn is_in_closed_set(&self, node: &AStarNode) -> bool {
		match self.closed_set.get(node) {
			Some(_)	=> true,
			None	=> false,
		}
	}

	/// Return a list of all the states which
	fn open_neighbours(&self, current: Rc<AStarNode>,
			game: &NPuzzle,
			heu: &HeuristicFn)
				-> Vec<RcASN> {
		let mut to_return = Vec::new();
		for act in Action::list_all() {
			if current.board().action_allowed(act.clone()) {
				let new_node = Rc::new(
						AStarNode::new(act, Some(current.clone()), game, heu));
				// TODO test if new_node in open_set with lower cost
				// TODO jump only if is_in_closed_set && ttl_cost < new_node.ttl_cost ?
				if self.is_in_closed_set(&*new_node) {
					continue ;
				}
				to_return.push(new_node);
			}
		}
		to_return
	}

	fn execute(&mut self, game: &NPuzzle, heu: &HeuristicFn)
			-> Option<RcASN> {
		let root = Rc::new(AStarNode::new_root(game, heu));
		self.open_set.push(root);
		while !self.open_set.is_empty() {
			self.nb_turn += 1;
			let current = self.open_set.pop().unwrap();

			// victory condition
			if game.is_complete(current.board()) {
				return Some(current);
			}

			let next = self.open_neighbours(current.clone(), game, heu);
			self.closed_set.insert(current, ());
			self.nb_state += next.len() as u32;
			self.open_set.extend(next);
		}
		None
	}

	pub fn solve(game: &NPuzzle, heu: &HeuristicFn) -> AStar
	{
		let mut to_return = AStar {
			board:		game.clone(),
			open_set:	BinaryHeap::new(),
			closed_set:	HashMap::new(),
			result:		None,
			nb_turn:	0,
			nb_state:	0,
		};
		to_return.result = to_return.execute(game, heu);
		to_return
	}

	pub fn print_result(&self) {
		match self.result {
			Some(ref x) => {
				let move_list = x.move_list();
				let mut board = self.board.get_initial_state().clone();
				println!("Initial state");
				println!("{}", board);
				for (i, action) in move_list.iter().enumerate() {
					board.execute_action(action.clone());
					println!("Move {} direction {}", i + 1, action);
					println!("{}", board);
				}
				println!("Number of turn  (complexity in time) {:?}",
						self.nb_turn);
				println!("Number of state (complexity in size) {:?}",
						self.nb_state);
			},
			None		=> println!("The puzzle is unsolvable"),
		}
	}
}

