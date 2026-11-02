use super::{Square,Piece};


#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub prom: Option<Piece>
}


impl Move {
    pub fn new(from: Square, to: Square, prom: Option<Piece>) -> Move {
        Move { from, to, prom }
    }

    pub fn from_uci(s: &str) -> Option<Move> {
        if s.len() != 4 && s.len() != 5 {
            return None;
        }

        let from = Square::from_uci(s.get(0..2)?)?;
        let to = Square::from_uci(s.get(2..4)?)?;

        let prom = match s.get(4..)? {
            ""  => None,
            "q" => Some(Piece::Queen),
            "r" => Some(Piece::Rook),
            "b" => Some(Piece::Bishop),
            "n" => Some(Piece::Knight),
            _ => return None,
        };

        Some(Move::new(from, to, prom))
    }

    pub fn to_uci(self) -> String {
        let mut all = String::with_capacity(5);
        all.push_str(&self.from.to_uci());
        all.push_str(&self.to.to_uci());
        if let Some(piece) = self.prom {
            all.push(piece.to_char());
        }
        all
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn sq(s: &str) -> Square {
        Square::from_uci(s).unwrap()
    }

    #[test]
    fn uci_round_trip_and_parsing() {
        for s in ["e2e4", "g1f3", "e7e8q", "a2a1n", "e1g1"] {
            assert_eq!(Move::from_uci(s).unwrap().to_uci(), s);
        }

        let m = Move::from_uci("e7e8q").unwrap();
        assert_eq!(m.from, sq("e7"));
        assert_eq!(m.to, sq("e8"));
        assert_eq!(m.prom, Some(Piece::Queen));
        assert_eq!(Move::from_uci("e2e4").unwrap().prom, None);
    }

    #[test]
    fn from_uci_rejects_malformed_input() {
        for s in ["", "e2", "e2e", "e2e4x", "e2e4qq", "e7e8k", "e7e8p", "e9e4", "é2e4", "e2e4é"] {
            assert!(Move::from_uci(s).is_none(), "{s:?}");
        }
    }
}