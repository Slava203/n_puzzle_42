//! A A* heuristic is a way to evaluate how much it will cost to evolve the
//! current state to the goal state. This is an approximation.

use npuzzle::{Board, Tile, NPuzzle};
use options;

pub type HeuristicFn = fn(current_state: &Board, game: &NPuzzle) -> i32;

pub fn from_option(opt: options::Heuristic) -> HeuristicFn {
	match opt {
		options::Heuristic::Manhattan =>		manhattan,
		options::Heuristic::MisplacedTiles =>	misplaced_tiles,
	}
}

pub fn manhattan(current_state: &Board, game: &NPuzzle) -> i32 {
	let mut to_return = 0;
	for (i, tile) in current_state.get_tiles().iter().enumerate() {
		let (x_goal, y_goal) = game.get_index_tiles()[tile.to_nbr() as usize];
		let (x_current, y_current) = current_state.xy_out_of_index(i);
		to_return += (x_current as i32 - x_goal as i32).abs() +
				(y_current as i32 - y_goal as i32).abs();
	}
	to_return
}

pub fn misplaced_tiles(current_state: &Board, game: &NPuzzle) -> i32 {
	let mut to_return = 0;
	for (i, current_tile) in current_state.get_tiles().iter().enumerate() {
		let goal_tile = &game.get_goal_state().get_tiles()[i];
		if current_tile.to_nbr() != goal_tile.to_nbr() {
			to_return += 1;
		}
	}
	to_return
}
