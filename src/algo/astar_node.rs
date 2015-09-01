// use npuzzle::action::{Action};
use std::cmp::Ordering;
use npuzzle::{Action, NPuzzle};
use algo::{Heuristic};
use npuzzle::{Board};

pub type ParentType = Option<Box<AStarNode>>;
// Type for recursive compute_board
// type RecBoard = Box<Board>;

#[derive(Debug)]
pub struct AStarNode
{
	///action made by this node
	action:			Action,

	///cost to come here (parent + 1)
	g:				i32,

	///cost to come to the end point (computed by heuristic)
	h:				i32,

	///previous node
	parent:			ParentType,

	///tiles
	current_state:	Board,
}

impl AStarNode
{
	// /// Recursive function which return the state of the game after the action
	// /// of this node has been executed.
	// ///
	// /// initial_state:	this is the state of the game before any action has
	// /// 				been executed.
	// fn compute_board(&self, initial_state: RecBoard) -> (i32, RecBoard) {
	// 	if !self.parent.is_none() {
	// 		(0, initial_state)
	// 	} else {
	// 		let (g, mut board) = self.parent.unwrap().compute_board(initial_state);
	// 		//the possibility to execute the action has already been tested
	// 		board.execute_action(self.action.clone());
	// 		(g + 1, board)
	// 	}
	// }

	// fn compute_cost(&self, game: &NPuzzle, heu: &Heuristic) -> (i32, i32) {
	// 	let mut boxed_board = Box::new(game.get_initial_state().clone());
	// 	let (g, current_state) =
	// 			self.compute_board(boxed_board);
	// 	let h = heu.h(&current_state, game.get_goal_state());
	// 	(g, h)
	// }

	fn compute_current_state(parent: &ParentType,
			game: &NPuzzle,
			action: Action) -> (i32, Board) {
		match *parent {
			Some(ref x)	=> {
				let mut board = x.current_state.clone();
				board.execute_action(action);
				(x.g + 1, board)
			},
			None		=> (0, game.get_initial_state().clone()),
		}
	}

	fn hidden_new(action: Action, parent: ParentType,
				game: &NPuzzle, heu: &Heuristic) -> AStarNode
	{
		let (g, board) =
			AStarNode::compute_current_state(&parent, game, action.clone());
		let mut to_return = AStarNode {
			action:			action,
			g:				0,
			h:				0,
			parent:			parent,
			current_state:	board,
		};
		// let (g, h) = to_return.compute_cost(game, heu);
		to_return.current_state = to_return.current_state;
		to_return.g = g;
		to_return.h = heu.h(&to_return.current_state, game.get_goal_state());
		to_return
	}

	pub fn new(action: Action, parent: ParentType,
				game: &NPuzzle, heu: &Heuristic) -> AStarNode
	{
		AStarNode::hidden_new(action, parent, game, heu)
	}

	pub fn new_root(game: &NPuzzle, heu: &Heuristic) -> AStarNode {
		AStarNode::hidden_new(Action::No, None, game, heu)
	}

	pub fn ttl_cost(&self) -> i32 {
		self.g + self.h
	}
}

impl PartialEq for AStarNode {
    fn eq(&self, other: &AStarNode) -> bool {
    	self.ttl_cost() == other.ttl_cost()
    }

    fn ne(&self, other: &AStarNode) -> bool {
    	!self.eq(other)
    }
}

impl Eq for AStarNode {}

impl Ord for AStarNode {
    fn cmp(&self, other: &AStarNode) -> Ordering {
        // Notice that the we flip the ordering here
        other.ttl_cost().cmp(&self.ttl_cost())
    }
}


impl PartialOrd for AStarNode {
    fn partial_cmp(&self, other: &AStarNode) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
