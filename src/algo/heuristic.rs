//! A A* heuristic is a way to evaluate how much it will cost to evolve the
//! current state to the goal state. This is an approximation.

use npuzzle::{Board, NPuzzle};
use options;

pub type HeuristicFn = fn(current_state: &Board, game: &NPuzzle) -> i32;

pub fn from_option(opt: options::Heuristic) -> HeuristicFn {
	match opt {
		options::Heuristic::Manhattan =>		manhattan,
		options::Heuristic::MisplacedTiles =>	misplaced_tiles,
		options::Heuristic::TilesOut =>			tiles_out,
	}
}

/// https://fr.wikipedia.org/wiki/Distance_de_Manhattan
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

/// http://heuristicswiki.wikispaces.com/Misplaced+Tiles
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

/// http://heuristicswiki.wikispaces.com/Tiles+out+of+row+and+column
pub fn tiles_out(current_state: &Board, game: &NPuzzle) -> i32 {
	let mut to_return = 0;
	for (i, tile) in current_state.get_tiles().iter().enumerate() {
		let (x_goal, y_goal) = game.get_index_tiles()[tile.to_nbr() as usize];
		let (x_current, y_current) = current_state.xy_out_of_index(i);
		if x_goal != x_current {
			to_return += 1;
		}
		if y_goal != y_current {
			to_return += 1;
		}
	}
	to_return
}
