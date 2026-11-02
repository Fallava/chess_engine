pub mod random;
pub mod minimax;

pub use random::BotRandom;
pub use minimax::BotMiniMax;

use crate::engine::{Move, Board};

pub trait BotTrait {
    fn play_move(&self, board: &Board, id: &str) -> Option<Move>;
}