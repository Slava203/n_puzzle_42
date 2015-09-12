mod npuzzle;
mod parser;
mod tile;
mod errors;
mod test;
mod board;
mod action;
mod test_check_solvability;

pub use self::npuzzle::{NPuzzle};
pub use self::tile::{Tile};
pub use self::board::{Board};
pub use self::action::{Action};
