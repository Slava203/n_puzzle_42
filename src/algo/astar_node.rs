// use npuzzle::action::{Action};
use std::rc::Rc;
use std::cmp::Ordering;
use npuzzle::{Action, NPuzzle};
use algo::{HeuristicFn};
use npuzzle::{Board};

pub type ParentType = Option<Rc<AStarNode>>;
// Type for recursive compute_board
// type RecBoard = Rc<Board>;

#[derive(Clone, Hash)]
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
	fn compute_current_state(parent: &ParentType,
			game: &NPuzzle,
			action: Action) -> (i32, Board) {
		match *parent {
			Some(ref x)	=> {
				let mut board = x.current_state.clone();
				// println!("#compute_current_state {:?}", action);
				// println!("board before\n{}", board);
				board.execute_action(action);
				// println!("board after\n{}", board);
				(x.g + 1, board)
			},
			None		=> (0, game.get_initial_state().clone()),
		}
	}

	fn hidden_new(action: Action, parent: ParentType,
				game: &NPuzzle, heu: &HeuristicFn) -> AStarNode
	{
		let (g, board) =
			AStarNode::compute_current_state(&parent, game, action.clone());
		let mut to_return = AStarNode {
			action:			action,
			g:				g,
			h:				0,
			parent:			parent,
			current_state:	board,
		};
		to_return.h = heu(&to_return.current_state, &game);
		to_return
	}

	pub fn new(action: Action, parent: ParentType,
				game: &NPuzzle, heu: &HeuristicFn) -> AStarNode
	{
		AStarNode::hidden_new(action, parent, game, heu)
	}

	pub fn new_root(game: &NPuzzle, heu: &HeuristicFn) -> AStarNode {
		AStarNode::hidden_new(Action::No, None, game, heu)
	}

	/// This function return cost * -1. It has no sense but the implementation
	/// of BinaryHeap always pop the biggest value (ie the biggest cost).
	pub fn invert_ttl_cost(&self) -> i32 {
		-(self.g + self.h)
	}

	pub fn ttl_cost(&self) -> i32 {
		self.g + self.h
	}

	pub fn board(&self) -> &Board {
		&self.current_state
	}

	/// Return the list of action of this node and of all its predecessors
	pub fn move_list(&self) -> Vec<Action> {
		if self.parent.is_none() {
			return Vec::new();
		}
		let mut action_lst = self.parent.as_ref()
				.map(|x| x.move_list()).unwrap();
		if self.action == Action::No {
			return action_lst;
		}
		action_lst.push(self.action.clone());
		action_lst
	}
}

impl PartialEq for AStarNode {
    fn eq(&self, other: &AStarNode) -> bool {
    	self.invert_ttl_cost() == other.invert_ttl_cost()
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

use std::fmt::{Formatter, Display, Error};
use std::fmt;

impl Display for AStarNode
{
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error>
	{
		let _ = write!(f, "ASN:{}:{}", self.action, self.ttl_cost());
		Ok(())
	}
}

impl fmt::Debug for AStarNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	write!(f, "{}", self)
    }
}
