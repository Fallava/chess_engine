use super::BotTrait;
use crate::engine::{Move, Board};

pub struct BotRandom;

impl BotTrait for BotRandom {
    fn play_move(&self, board: &Board, id: &str) -> Option<Move> {
        let moves = board.legal_moves();

        if moves.len() == 0 {
            return None
        }

        let choice = rand::random_range(..moves.len());
        println!("ID: {}   Make random choice of {} out of {}", id, choice, moves.len());
        Some(moves[choice])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::{Piece, Color};

    #[test]
    fn random_games_keep_both_kings_and_never_panic() {
        let bot = BotRandom;
        for _ in 0..100 {
            let mut board = Board::startpos();
            for _ in 0..300 {
                let Some(mv) = bot.play_move(&board,"10") else { break }; // mat ou pat
                board.make_move(mv);
                assert_eq!(board.pieces(Color::White, Piece::King).count(), 1);
                assert_eq!(board.pieces(Color::Black, Piece::King).count(), 1);
            }
        }
    }
}