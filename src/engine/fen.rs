use std::fmt;
use super::board::{Board, CastlingRights};
use super::{Color, Piece, Square};

#[derive(Debug, PartialEq, Eq)]
pub enum FenError {
    WrongFieldCount(usize),
    WrongRankCount(usize),
    InvalidPiece(char),
    RankTooLong(usize),
    RankTooShort(usize),
    InvalidTurn(String),
    InvalidCastling(String),
    InvalidEnPassant(String),
    InvalidNumber(String),
}

impl fmt::Display for FenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FenError::WrongFieldCount(n) => write!(f, "FEN: 6 champs attendus, {n} trouvés"),
            FenError::WrongRankCount(n) => write!(f, "FEN: 8 rangées attendues, {n} trouvées"),
            FenError::InvalidPiece(c) => write!(f, "FEN: pièce inconnue {c:?}"),
            FenError::RankTooLong(r) => write!(f, "FEN: la rangée {r} dépasse 8 cases"),
            FenError::RankTooShort(r) => write!(f, "FEN: la rangée {r} a moins de 8 cases"),
            FenError::InvalidTurn(s) => write!(f, "FEN: trait invalide {s:?}"),
            FenError::InvalidCastling(s) => write!(f, "FEN: roque invalide {s:?}"),
            FenError::InvalidEnPassant(s) => write!(f, "FEN: prise en passant invalide {s:?}"),
            FenError::InvalidNumber(s) => write!(f, "FEN: nombre invalide {s:?}"),
        }
    }
}

impl std::error::Error for FenError {}

impl Board {
    pub const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    pub fn startpos() -> Board {
        Board::from_fen(Self::START_FEN).expect("FEN de départ valide")
    }

    pub fn from_fen(fen: &str) -> Result<Board,FenError>{
        let fields: Vec<&str> = fen.split_whitespace().collect();
        let &[placement, turn, castling, en_passant, halfmove, fullmove] = fields.as_slice() else {
            return Err(FenError::WrongFieldCount(fields.len()));
        };

        let mut b = Board::empty();
        parse_placement(&mut b, placement)?;
        b.turn = parse_turn(turn)?;
        b.castling = parse_castlings(castling)?;
        b.en_passant = parse_en_passant(en_passant)?;
        b.halfmove = parse_number(halfmove)?;
        b.fullmove = parse_number(fullmove)?;

        Ok(b)
    }
}


fn parse_turn(turn: &str) -> Result<Color,FenError> {
    match turn {
        "w" => Ok(Color::White),
        "b" => Ok(Color::Black),
        _ => Err(FenError::InvalidTurn(turn.to_string()))
    }
}

fn parse_castlings(castling: &str) -> Result<CastlingRights,FenError> {
    let mut castle = CastlingRights::EMPTY;
    
    if castling == "-" {
        return Ok(CastlingRights::EMPTY)
    }

    for c in castling.chars() {
        match c {
            'K' => { castle.white_kingside = true }
            'Q' => { castle.white_queenside = true }
            'k' => { castle.black_kingside = true }
            'q' => { castle.black_queenside = true }
            _ => return Err(FenError::InvalidCastling(castling.to_string()))
        }
    }
    Ok(castle)
}

fn parse_en_passant(text: &str) -> Result<Option<Square>, FenError> {
    if text == "-" {
        return Ok(None);
    }
    match Square::from_uci(text) {
        // rangée "3" = 2 et rangée "6" = 5 en numérotation interne
        Some(sq) if sq.rank() == 2 || sq.rank() == 5 => Ok(Some(sq)),
        _ => Err(FenError::InvalidEnPassant(text.to_string())),
    }
}

fn parse_number(text: &str) -> Result<u16, FenError> {
    text.parse().map_err(|_| FenError::InvalidNumber(text.to_string()))
}

fn parse_placement(board: &mut Board, placement: &str) -> Result<(), FenError> {
    let ranks: Vec<&str> = placement.split('/').collect();
    if ranks.len() != 8 {
        return Err(FenError::WrongRankCount(ranks.len()));
    }

    for (i, rank_text) in ranks.iter().enumerate() {
        let rank = 7 - i; // la 1re rangée du texte est la rangée 8
        let mut file = 0usize;

        for c in rank_text.chars() {
            // un chiffre = autant de cases vides
            if let Some(n) = c.to_digit(10) {
                if n == 0 {
                    return Err(FenError::InvalidPiece(c));
                }
                file += n as usize;
                if file > 8 {
                    return Err(FenError::RankTooLong(rank));
                }
                continue;
            }

            // sinon une pièce : minuscule = noir, majuscule = blanc
            let piece = Piece::from_char(c.to_ascii_lowercase()).ok_or(FenError::InvalidPiece(c))?;
            let color = if c.is_ascii_uppercase() { Color::White } else { Color::Black };

            if file >= 8 {
                return Err(FenError::RankTooLong(rank));
            }
            let Some(sq) = Square::new((rank * 8 + file) as u8) else {
                return Err(FenError::RankTooLong(rank)); // inatteignable : rank et file sont < 8
            };
            board.put_piece(color, piece, sq);
            file += 1;
        }

        if file < 8 {
            return Err(FenError::RankTooShort(rank));
        }
    }
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_fen_startpos() {
        let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        assert_eq!(board.occupied().count(), 32);
        assert_eq!(board.piece_at(Square::from_uci("e1").unwrap()), Some((Color::White, Piece::King)));
        assert_eq!(board.piece_at(Square::from_uci("d8").unwrap()), Some((Color::Black, Piece::Queen)));
        assert_eq!(board.turn, Color::White);
        let mut full = CastlingRights::EMPTY;
        full.white_kingside = true;
        full.white_queenside = true;
        full.black_kingside = true;
        full.black_queenside = true;

        assert_eq!(board.castling, full);
        assert_eq!(board.en_passant, None);
    }

    #[test]
    fn from_fen_rejects_garbage() {
        assert!(Board::from_fen("").is_err());
        assert!(Board::from_fen("rnbqkbnr/pppppppp w").is_err());
    }

    #[test]
    fn from_fen_reads_turn_and_en_passant() {
        let b = Board::from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1").unwrap();
        assert_eq!(b.turn, Color::Black);
        assert_eq!(b.en_passant, Some(Square::from_uci("e3").unwrap()));
        assert_eq!(b.piece_at(Square::from_uci("e4").unwrap()), Some((Color::White, Piece::Pawn)));
    }

    #[test]
    fn from_fen_reports_the_right_error() {
        let start = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
        let cases = [
            ("".to_string(), FenError::WrongFieldCount(0)),
            (format!("{start} w KQkq -"), FenError::WrongFieldCount(4)),
            ("8/8/8 w - - 0 1".to_string(), FenError::WrongRankCount(3)),
            (format!("{start} x KQkq - 0 1"), FenError::InvalidTurn("x".into())),
            (format!("{start} w KQkz - 0 1"), FenError::InvalidCastling("KQkz".into())),
            (format!("{start} w KQkq e5 0 1"), FenError::InvalidEnPassant("e5".into())),
            (format!("{start} w KQkq - a 1"), FenError::InvalidNumber("a".into())),
            ("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNX w KQkq - 0 1".to_string(), FenError::InvalidPiece('X')),
            ("rnbqkbnr/ppppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(), FenError::RankTooLong(6)),
            ("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBN w KQkq - 0 1".to_string(), FenError::RankTooShort(0)),
        ];
        for (fen, expected) in cases {
            assert_eq!(Board::from_fen(&fen).unwrap_err(), expected, "fen {fen:?}");
        }
    }

    #[test]
    fn from_fen_accepts_no_castling_and_no_en_passant() {
        let b = Board::from_fen("8/8/8/8/8/8/8/K6k w - - 0 1").unwrap();
        assert_eq!(b.castling, CastlingRights::EMPTY);
        assert_eq!(b.en_passant, None);
        assert_eq!(b.occupied().count(), 2);
    }
}