//! A A* heuristic is a way to evaluate how much it will cost to evolve the
//! current state to the goal state. This is an approximation.

use npuzzle::{Board, Tile};
use options;

pub type HeuristicFn = fn(current_state: &Board, goal_state: &Board) -> i32;

pub fn from_option(opt: options::Heuristic) -> HeuristicFn {
	match opt {
		options::Heuristic::Manhattan =>	manhattan,
	}
}

pub fn manhattan(current_state: &Board, goal_state: &Board) -> i32 {
	let mut to_return = 0;
	for (i, tile) in current_state.get_tiles().iter().enumerate() {
		let (x_goal, y_goal) = match tile.to_nbr() - 1 {
			-1 =>	current_state
							.xy_out_of_index(current_state.nb_tile() - 1),
			_ =>	current_state.xy_out_of_index(tile.to_nbr() as usize - 1),
		};
		let (x_current, y_current) = current_state.xy_out_of_index(i);
		to_return += (x_current as i32 - x_goal as i32).abs() +
				(y_current as i32 - y_goal as i32).abs();
	}
	to_return
}
