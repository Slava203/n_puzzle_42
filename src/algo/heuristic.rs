use npuzzle::{Board};

/// A A* heuristic is a way to evaluate how much it will cost to evolve the
/// current state to the goal state.
pub trait Heuristic {
	fn h(&self, current_state: &Board, goal_state: &Board) -> i32;
}
