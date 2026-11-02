use super::attacks::*;
use super::{Board, Color, Piece, Square};

impl Board {
    pub fn king_square(&self, color: Color) -> Option<Square>{
        self.pieces(color, Piece::King).lsb()
    }

    pub fn is_attacked(&self, sq: Square, by: Color) -> bool {
        let occupied = self.occupied();

        let king = self.pieces(by, Piece::King);
        let knight = self.pieces(by, Piece::Knight);
        let pawn = self.pieces(by, Piece::Pawn);
        let diago = self.pieces(by, Piece::Queen) | self.pieces(by, Piece::Bishop);
        let straight = self.pieces(by, Piece::Queen) | self.pieces(by, Piece::Rook);

        !(king_attacks(sq) & king).is_empty()
            || !(knight_attacks(sq) & knight).is_empty()
            || !(pawn_attacks(by.opposite(), sq) & pawn).is_empty()
            || !(bishop_attacks(sq, occupied) & diago).is_empty()
            || !(rook_attacks(sq, occupied) & straight).is_empty()
    }

    pub fn in_check(&self) -> bool {
        match self.king_square(self.turn) {
            Some(king) => self.is_attacked(king, self.turn.opposite()),
            None => false
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn in_check(fen: &str) -> bool {
        Board::from_fen(fen).unwrap().in_check()
    }

    #[test]
    fn detects_each_kind_of_attacker_and_blockers() {
        // position de départ : pas d'échec
        assert!(!in_check("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"));

        assert!(in_check("4k3/8/8/8/8/5n2/8/4K3 w - - 0 1"));   // cavalier f3 attaque e1
        assert!(in_check("4k3/8/8/8/8/8/3p4/4K3 w - - 0 1"));   // pion noir d2 attaque e1
        assert!(in_check("k7/8/8/8/7b/8/8/4K3 w - - 0 1"));     // fou h4 attaque e1 (diagonale)
        assert!(in_check("k3r3/8/8/8/8/8/8/4K3 w - - 0 1"));    // tour e8 attaque e1 (colonne)

        // un bloqueur coupe l'attaque, et un pion blanc en d2 n'attaque pas e1
        assert!(!in_check("k3r3/8/8/8/8/8/4P3/4K3 w - - 0 1"));
        assert!(!in_check("4k3/8/8/8/8/8/3P4/4K3 w - - 0 1"));
    }
}