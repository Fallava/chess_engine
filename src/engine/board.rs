use super::Color;
use super::Piece;
use super::Bitboard;
use super::Square;

#[derive(Clone, Copy,PartialEq, Eq, Debug)]
pub struct CastlingRights {
    pub white_kingside: bool,
    pub white_queenside: bool,
    pub black_kingside: bool,
    pub black_queenside: bool,
}

impl CastlingRights {
    pub const EMPTY: Self = CastlingRights{
        white_kingside: false,
        white_queenside: false,
        black_kingside: false,
        black_queenside: false
    };

    
}


#[derive(Clone, Debug)]
pub struct Board {
    pieces: [[Bitboard; 6]; 2],
    by_color: [Bitboard; 2],
    occupied: Bitboard,

    pub turn: Color,
    pub castling: CastlingRights,
    pub en_passant: Option<Square>,
    pub halfmove: u16,
    pub fullmove: u16,
}

impl Board {
    pub fn empty() -> Board {
        Board {
            pieces: [[Bitboard::EMPTY;6];2],
            by_color: [Bitboard::EMPTY;2],
            occupied: Bitboard::EMPTY,

            turn: Color::White,
            castling: CastlingRights::EMPTY,
            en_passant: None,
            halfmove: 0,
            fullmove: 1
        }
    }

    pub fn pieces(&self, color: Color, piece: Piece) -> Bitboard {
        self.pieces[color.index()][piece.index()]
    }

    pub fn color_bb(&self, color: Color) -> Bitboard {
        self.by_color[color.index()]
    }
    pub fn occupied(&self) -> Bitboard {
        self.occupied
    }

    pub fn put_piece(&mut self, color: Color, piece: Piece, sq: Square) {
        debug_assert!(!self.occupied.contains(sq), "case {sq:?} déjà occupée");
        self.pieces[color.index()][piece.index()].insert(sq);
        self.by_color[color.index()].insert(sq);
        self.occupied.insert(sq);
    }
    pub fn remove_piece(&mut self, sq: Square) -> Option<(Color, Piece)> {
        let (color, piece) = self.piece_at(sq)?;   // case vide : on sort avec None
        self.pieces[color.index()][piece.index()].remove(sq);
        self.by_color[color.index()].remove(sq);
        self.occupied.remove(sq);
        Some((color, piece))
    }

    pub fn piece_at(&self, sq: Square) -> Option<(Color, Piece)> {
        if !self.occupied.contains(sq) {return None}

        let color = if self.by_color[Color::White.index()].contains(sq) {
            Color::White 
        } else {
            Color::Black
        };

        for piece in Piece::ALL {
            if self.pieces[color.index()][piece.index()].contains(sq) {
                return Some((color, piece))
            }
        }
        None
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn put_then_piece_at_then_remove() {
        let mut board = Board::empty();
        let e4 = Square::new(28).unwrap();
        assert_eq!(board.piece_at(e4), None);

        board.put_piece(Color::White, Piece::Knight, e4);
        assert_eq!(board.piece_at(e4), Some((Color::White, Piece::Knight)));
        assert_eq!(board.pieces(Color::White, Piece::Knight), Bitboard::from_square(e4));
        assert_eq!(board.occupied(), Bitboard::from_square(e4));

        assert_eq!(board.remove_piece(e4), Some((Color::White, Piece::Knight)));
        assert_eq!(board.piece_at(e4), None);
        assert_eq!(board.occupied(), Bitboard::EMPTY);
        assert_eq!(board.remove_piece(e4), None);   // retirer une case vide ne fait rien
    }

    #[test]
    fn colors_and_occupied_stay_consistent() {
        let mut board = Board::empty();
        let a2 = Square::new(8).unwrap();
        let h8 = Square::new(63).unwrap();
        board.put_piece(Color::White, Piece::Pawn, a2);
        board.put_piece(Color::Black, Piece::Rook, h8);

        assert_eq!(board.color_bb(Color::White), Bitboard::from_square(a2));
        assert_eq!(board.color_bb(Color::Black), Bitboard::from_square(h8));
        assert_eq!(board.occupied(), Bitboard::from_square(a2) | Bitboard::from_square(h8));
    }
}