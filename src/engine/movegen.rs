use super::attacks::*;
use super::{Board, Move, Piece, Color, Bitboard, Square};


impl Board {
    fn king_moves(&self, moves: &mut Vec<Move>) {
        let us = self.turn;
        let own = self.color_bb(us);

        for from in self.pieces(us, Piece::King) {
            let target = king_attacks(from) & !own;
            for to in target {
                moves.push(Move::new(from, to, None));
            }
        }
    }

    fn queen_moves(&self, moves: &mut Vec<Move>) {
        let us = self.turn;
        let own = self.color_bb(us);

        for from in self.pieces(us, Piece::Queen) {
            let target = queen_attacks(from, self.occupied()) & !own;
            for to in target {
                moves.push(Move::new(from, to, None));
            }
        }
    }

    fn rook_moves(&self, moves: &mut Vec<Move>) {
        let us = self.turn;
        let own = self.color_bb(us);

        for from in self.pieces(us, Piece::Rook) {
            let target = rook_attacks(from, self.occupied()) & !own;
            for to in target {
                moves.push(Move::new(from, to, None));
            }
        }
    }

    fn bishop_moves(&self, moves: &mut Vec<Move>) {
        let us = self.turn;
        let own = self.color_bb(us);

        for from in self.pieces(us, Piece::Bishop) {
            let target = bishop_attacks(from, self.occupied()) & !own;
            for to in target {
                moves.push(Move::new(from, to, None));
            }
        }
    }

    fn knight_moves(&self, moves: &mut Vec<Move>) {
        let us = self.turn;
        let own = self.color_bb(us);

        for from in self.pieces(us, Piece::Knight) {
            let target = knight_attacks(from) & !own;
            for to in target {
                moves.push(Move::new(from, to, None));
            }
        }
    }

    fn pawn_moves(&self, moves: &mut Vec<Move>) {
        let us = self.turn;
        let empty = !self.occupied();

        let (start_rank, promo_rank) = match us {
            Color::White => (1,7),
            Color::Black => (6,0)
        };

        let forward = |b: Bitboard| match us {
            Color::White => b.north(),
            Color::Black => b.south(),
        };

        let mut target = self.color_bb(us.opposite());
        if let Some(ep) = self.en_passant { target.insert(ep) }

        for from in self.pieces(us, Piece::Pawn) {
            let one = forward(Bitboard::from_square(from)) & empty;

            for to in one {
                push_pawn_move(moves, from, to, promo_rank);
            }

            if from.rank() == start_rank {
                let two = forward(one) & empty;
                for to in two {
                    moves.push(Move::new(from, to, None));
                }
            }

            for to in target & pawn_attacks(us, from) {
                push_pawn_move(moves, from, to, promo_rank);
            } 
        }
    }

    fn castle_moves(&self, moves: &mut Vec<Move>) {
        let us = self.turn;
        let them = us.opposite();

        // (droit petit roque, droit grand roque, index de la première case de la rangée de base)
        let (kingside, queenside, base) = match us {
            Color::White => (self.castling.white_kingside, self.castling.white_queenside, 0u8),
            Color::Black => (self.castling.black_kingside, self.castling.black_queenside, 56u8),
        };
        let sq = |file: u8| Square::new(base + file).unwrap(); // la case de cette colonne sur la rangée de base
        let occupied = self.occupied();

        // le roi doit être sur sa case (colonne 4) et ne pas être en échec
        if !self.pieces(us, Piece::King).contains(sq(4)) || self.is_attacked(sq(4), them) {
            return;
        }

        // petit roque : tour en colonne 7, cases f et g (5 et 6) vides, f non attaquée
        if kingside
            && self.pieces(us, Piece::Rook).contains(sq(7))
            && !occupied.contains(sq(5))
            && !occupied.contains(sq(6))
            && !self.is_attacked(sq(5), them)
        {
            moves.push(Move::new(sq(4), sq(6), None));
        }

        if queenside
            && self.pieces(us, Piece::Rook).contains(sq(0))
            && !occupied.contains(sq(1))
            && !occupied.contains(sq(2))
            && !occupied.contains(sq(3))
            && !self.is_attacked(sq(3), them) 
        {
            moves.push(Move::new(sq(4), sq(2), None));
        }
    }

    pub fn pseudo_legal_moves(&self) -> Vec<Move> {
        let mut moves = Vec::with_capacity(64);
        self.pawn_moves(&mut moves);
        self.knight_moves(&mut moves);
        self.bishop_moves(&mut moves);
        self.rook_moves(&mut moves);
        self.queen_moves(&mut moves);
        self.king_moves(&mut moves);
        self.castle_moves(&mut moves);
        moves
    }

    pub fn legal_moves(&self) -> Vec<Move> {
        let us = self.turn; // à lire AVANT make_move : le trait change après
        let mut legal = Vec::with_capacity(48);

        for mv in self.pseudo_legal_moves() {
            let mut next = self.clone();
            next.make_move(mv);

            let safe = match next.king_square(us) {
                Some(king) => !next.is_attacked(king, us.opposite()),
                None => true,
            };
            if safe {
                legal.push(mv);
            }
        }
        legal
    }
}



fn push_pawn_move(moves: &mut Vec<Move>, from: Square, to: Square, promo_rank: u8) {
    if to.rank() == promo_rank {
        for piece in [Piece::Queen, Piece::Rook, Piece::Bishop, Piece::Knight] {
            moves.push(Move::new(from, to, Some(piece)));
        }
    } else {
        moves.push(Move::new(from, to, None));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn knight_moves_from_the_start_position() {
        let b = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        let mut moves = Vec::new();
        b.knight_moves(&mut moves);
        let mut uci: Vec<String> = moves.iter().map(|m| m.to_uci()).collect();
        uci.sort();
        assert_eq!(uci, ["b1a3", "b1c3", "g1f3", "g1h3"]);
    }

    #[test]
    fn knight_can_capture_but_not_take_its_own_piece() {
        // cavalier blanc e4 ; pion blanc en c3 (bloque) et pion noir en f6 (capturable)
        let b = Board::from_fen("4k3/8/5p2/8/4N3/2P5/8/4K3 w - - 0 1").unwrap();
        let mut moves = Vec::new();
        b.knight_moves(&mut moves);
        let uci: Vec<String> = moves.iter().map(|m| m.to_uci()).collect();
        assert!(uci.contains(&"e4f6".to_string()));     // capture permise
        assert!(!uci.contains(&"e4c3".to_string()));    // sa propre pièce
        assert_eq!(uci.len(), 7);                       // 8 cases - la case alliée
    }

        fn pawn_uci(fen: &str) -> Vec<String> {
        let b = Board::from_fen(fen).unwrap();
        let mut moves = Vec::new();
        b.pawn_moves(&mut moves);
        let mut uci: Vec<String> = moves.iter().map(|m| m.to_uci()).collect();
        uci.sort();
        uci
    }

    #[test]
    fn pawn_pushes_and_blocking() {
        // départ : 8 poussées simples + 8 doubles
        assert_eq!(pawn_uci("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").len(), 16);
        // pion e2 bloqué par un pion noir en e3 : ni simple ni double
        assert!(pawn_uci("4k3/8/8/8/8/4p3/4P3/4K3 w - - 0 1").is_empty());
        // pion noir : e7e5 et e7e6
        assert_eq!(pawn_uci("4k3/4p3/8/8/8/8/8/4K3 b - - 0 1"), ["e7e5", "e7e6"]);
    }

    #[test]
    fn pawn_promotions_and_en_passant() {
        // promotion simple : 4 coups
        assert_eq!(
            pawn_uci("4k3/P7/8/8/8/8/8/4K3 w - - 0 1"),
            ["a7a8b", "a7a8n", "a7a8q", "a7a8r"]
        );
        // avec un cavalier noir en b8 : 4 promotions en poussant + 4 en capturant
        assert_eq!(pawn_uci("1n2k3/P7/8/8/8/8/8/4K3 w - - 0 1").len(), 8);
        // prise en passant : e5 peut pousser en e6 ou prendre en d6
        assert_eq!(pawn_uci("4k3/8/8/3pP3/8/8/8/4K3 w - d6 0 1"), ["e5d6", "e5e6"]);
    }

        fn castle_uci(fen: &str) -> Vec<String> {
        let b = Board::from_fen(fen).unwrap();
        let mut moves = Vec::new();
        b.castle_moves(&mut moves);
        let mut uci: Vec<String> = moves.iter().map(|m| m.to_uci()).collect();
        uci.sort();
        uci
    }

    #[test]
    fn castling_rules() {
        // tout est libre : les deux roques, pour chaque couleur
        assert_eq!(castle_uci("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1"), ["e1c1", "e1g1"]);
        assert_eq!(castle_uci("r3k2r/8/8/8/8/8/8/R3K2R b KQkq - 0 1"), ["e8c8", "e8g8"]);
        // pas de droit
        assert!(castle_uci("r3k2r/8/8/8/8/8/8/R3K2R w - - 0 1").is_empty());
        // un fou en f1 bloque le petit roque
        assert_eq!(castle_uci("r3k2r/8/8/8/8/8/8/R3KB1R w KQkq - 0 1"), ["e1c1"]);
        // une tour noire en f8 attaque f1 : le petit roque est interdit
        assert_eq!(castle_uci("5r1k/8/8/8/8/8/8/R3K2R w KQ - 0 1"), ["e1c1"]);
        // le roi est en échec (tour e8) : aucun roque
        assert!(castle_uci("4r2k/8/8/8/8/8/8/R3K2R w KQ - 0 1").is_empty());
        // b1 attaquée (tour b8) ne gêne pas le grand roque
        assert_eq!(castle_uci("1r5k/8/8/8/8/8/8/R3K2R w KQ - 0 1"), ["e1c1", "e1g1"]);
        // un cavalier en b1 bloque le grand roque, même si le roi ne traverse pas b1
        assert_eq!(castle_uci("r3k2r/8/8/8/8/8/8/RN2K2R w KQkq - 0 1"), ["e1g1"]);
    }

    #[test]
    fn legal_move_counts_and_pins() {
        let count = |fen: &str| Board::from_fen(fen).unwrap().legal_moves().len();

        assert_eq!(count("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"), 20);
        // Kiwipete : 48 coups (valeur publiée)
        assert_eq!(count("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1"), 48);

        // cavalier cloué en e2 par la tour e8 : seul le roi peut jouer
        let b = Board::from_fen("4r2k/8/8/8/8/8/4N3/4K3 w - - 0 1").unwrap();
        let legal = b.legal_moves();
        assert_eq!(legal.len(), 4);
        assert!(legal.iter().all(|m| m.from.to_uci() == "e1"));
    }
}