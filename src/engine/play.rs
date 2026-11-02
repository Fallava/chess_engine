
use super::Board;
use super::Move;
use super::Piece;
use super::Square;
use super::Color;


impl Board {
    pub fn make_move(&mut self, mv: Move) {
        let us = self.turn;
        let old_en_passant = self.en_passant;

        let (_, piece) = self.remove_piece(mv.from).unwrap();
        let captured = self.remove_piece(mv.to);

        // prise en passant : un pion qui change de colonne vers la case de prise en passant
        let is_en_passant = piece == Piece::Pawn
            && Some(mv.to) == old_en_passant
            && mv.from.file() != mv.to.file();
        if is_en_passant {
            let sq = Square::new(mv.from.rank() * 8 + mv.to.file()).unwrap();
            self.remove_piece(sq);
        }

        self.put_piece(us, mv.prom.unwrap_or(piece), mv.to);

        // roque : la tour saute de l'autre côté du roi
        if piece == Piece::King && mv.from.file().abs_diff(mv.to.file()) == 2 {
            let rank = mv.from.rank();
            let (rook_from, rook_to) = if mv.to.file() == 6 { (7, 5) } else { (0, 3) };
            let rook_from = Square::new(rank * 8 + rook_from).unwrap();
            let rook_to = Square::new(rank * 8 + rook_to).unwrap();
            self.remove_piece(rook_from);
            self.put_piece(us, Piece::Rook, rook_to);
        }

        // droits de roque : on regarde départ ET arrivée,
        // ça couvre « la tour part » et « la tour est capturée »
        self.lose_castling_rights(mv.from);
        self.lose_castling_rights(mv.to);

        // double poussée : la case sautée est à mi-chemin
        self.en_passant = if piece == Piece::Pawn && mv.from.rank().abs_diff(mv.to.rank()) == 2 {
            Square::new(((mv.from.index() + mv.to.index()) / 2) as u8)
        } else {
            None
        };

        // règle des 50 coups : remise à zéro sur coup de pion ou capture
        if piece == Piece::Pawn || captured.is_some() || is_en_passant {
            self.halfmove = 0;
        } else {
            self.halfmove += 1;
        }

        if us == Color::Black {
            self.fullmove += 1;
        }
        self.turn = us.opposite();
    }

    fn lose_castling_rights(&mut self, sq: Square) {
        match sq.index() {
            0 => self.castling.white_queenside = false,
            7 => self.castling.white_kingside = false,
            4 => {
                self.castling.white_kingside = false;
                self.castling.white_queenside = false;
            }
            56 => self.castling.black_queenside = false,
            63 => self.castling.black_kingside = false,
            60 => {
                self.castling.black_kingside = false;
                self.castling.black_queenside = false;
            }
            _ => {}
        }
    }
}