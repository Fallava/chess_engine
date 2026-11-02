
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Piece {
    King,
    Queen,
    Pawn,
    Rook,
    Bishop,
    Knight,
}

impl Piece {
    pub const ALL: [Piece; 6] = [
        Piece::Pawn, Piece::Knight, Piece::Bishop,
        Piece::Rook, Piece::Queen, Piece::King,
    ];

    pub fn index(self) -> usize {
        self as usize
    }
    
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'k' => Some(Piece::King),
            'q' => Some(Piece::Queen),
            'p' => Some(Piece::Pawn),
            'r' => Some(Piece::Rook),
            'b' => Some(Piece::Bishop),
            'n' => Some(Piece::Knight),
            _ => None
        }
    }

    pub fn to_char(self) -> char {
        match self {
            Piece::King     => 'k',
            Piece::Queen    => 'q',
            Piece::Pawn     => 'p',
            Piece::Rook     => 'r',
            Piece::Bishop   => 'b',
            Piece::Knight   => 'n',
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALL: [Piece; 6] = [
        Piece::Pawn,
        Piece::Knight,
        Piece::Bishop,
        Piece::Rook,
        Piece::Queen,
        Piece::King,
    ];

    #[test]
    fn know_char_to_piece() {
        assert_eq!(Piece::from_char('k'), Some(Piece::King));
        assert_eq!(Piece::from_char('q'), Some(Piece::Queen));
        assert_eq!(Piece::from_char('p'), Some(Piece::Pawn));
        assert_eq!(Piece::from_char('r'), Some(Piece::Rook));
        assert_eq!(Piece::from_char('b'), Some(Piece::Bishop));
        assert_eq!(Piece::from_char('n'), Some(Piece::Knight));
    }

    #[test]
    fn reject_other_char() {
        let characters = ['B',' ','i','o','K','P'];
        for c in characters {
            assert_eq!(Piece::from_char(c), None, "char : {c}");
        }
    }

    #[test]
    fn piece_to_char() {
        assert_eq!(Piece::King.to_char(), 'k');
        assert_eq!(Piece::Queen.to_char(), 'q');
        assert_eq!(Piece::Pawn.to_char(), 'p');
        assert_eq!(Piece::Rook.to_char(), 'r');
        assert_eq!(Piece::Bishop.to_char(), 'b');
        assert_eq!(Piece::Knight.to_char(), 'n');
    }

    #[test]
    fn comeback_know_char() {
        let characters = ['k','q','p','r','b','n'];
        for c in characters {
            assert_eq!(Piece::from_char(c).unwrap().to_char(), c, "char : {c}")
        }
    }

        #[test]
    fn from_char_accepts_exactly_kqprbn_among_ascii() {
        for byte in 0u8..128 {
            let c = byte as char;
            assert_eq!(
                Piece::from_char(c).is_some(),
                "kqprbn".contains(c),
                "char {c:?}"
            );
        }
    }

    #[test]
    fn from_char_inverts_to_char_for_all_pieces() {
        for piece in ALL {
            assert_eq!(Piece::from_char(piece.to_char()), Some(piece), "piece {piece:?}");
        }
    }

    #[test]
    fn to_char_gives_a_different_char_for_each_piece() {
        let mut seen = std::collections::HashSet::new();
        for piece in ALL {
            assert!(seen.insert(piece.to_char()), "doublon pour {piece:?}");
        }
        assert_eq!(seen.len(), 6);
    }

    // dans piece.rs
    #[test]
    fn piece_indices_are_0_to_5_without_duplicates() {
        let mut indices: Vec<usize> = Piece::ALL.iter().map(|p| p.index()).collect();
        indices.sort();
        assert_eq!(indices, vec![0, 1, 2, 3, 4, 5]);
    }
}