#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Square(u8);

impl Square {
    pub fn new(index: u8) -> Option<Square> {
        if index < 64 {
            Some(Square(index))
        } else {
            None
        }
    }

    pub fn index(self) -> usize {
        self.0 as usize
    }

    pub fn file(self) -> u8 {
        self.0 % 8
    }

    pub fn rank(self) -> u8 {
        self.0 / 8
    }

    pub fn to_uci(self) -> String {
        let letter = (b'a' + self.file()) as char;
        let digit = (b'1' + self.rank()) as char;
        format!("{letter}{digit}")
    }

    pub fn from_uci(s: &str) -> Option<Square> {
        // Exactement 2 octets, sinon None. Aucun accès par index : impossible de paniquer.
        let &[file, rank] = s.as_bytes() else {
            return None;
        };

        let file = file.wrapping_sub(b'a');
        let rank = rank.wrapping_sub(b'1');

        if file < 8 && rank < 8 {
            Some(Square(rank * 8 + file))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square_accept_0_to_63() {
        for i in 0..64 {
            assert!(Square::new(i).is_some(), "index {i}")
        }
    }

    #[test]
    fn square_reject_above_63() {
        assert!(Square::new(64).is_none());
        assert!(Square::new(255).is_none());
    }

    #[test]
    fn index_returns_the_inner_value() {
        assert_eq!(Square::new(0).unwrap().index(), 0);
        assert_eq!(Square::new(28).unwrap().index(), 28);
        assert_eq!(Square::new(63).unwrap().index(), 63);
    }

    #[test]
    fn file_and_rank_of_a1() {
        let a1 = Square::new(0).unwrap();
        assert_eq!((a1.file(), a1.rank()), (0, 0));
    }

    #[test]
    fn file_and_rank_of_h1() {
        let h1 = Square::new(7).unwrap();
        assert_eq!((h1.file(), h1.rank()), (7, 0));
    }

    #[test]
    fn file_and_rank_of_a2() {
        let a2 = Square::new(8).unwrap();
        assert_eq!((a2.file(), a2.rank()), (0, 1));
    }

    #[test]
    fn file_and_rank_of_e4() {
        let e4 = Square::new(28).unwrap();
        assert_eq!((e4.file(), e4.rank()), (4, 3));
    }

    #[test]
    fn file_and_rank_of_h8() {
        let h8 = Square::new(63).unwrap();
        assert_eq!((h8.file(), h8.rank()), (7, 7));
    }

    #[test]
    fn file_and_rank_rebuild_the_index_for_all_squares() {
        for i in 0..64u8 {
            let sq = Square::new(i).unwrap();
            assert_eq!(sq.rank() * 8 + sq.file(), i, "case {i}");
        }
    }

    #[test]
    fn to_uci_of_known_squares() {
        let cases = [(0, "a1"), (7, "h1"), (28, "e4"), (56, "a8"), (63, "h8")];
        for (index, expected) in cases {
            assert_eq!(
                Square::new(index).unwrap().to_uci(),
                expected,
                "index {index}"
            );
        }
    }

    #[test]
    fn to_uci_0_to_63() {
        for il in 0..8 {
            for ic in 0..8 {
                let l = (b'a' + il) as char;
                let c = (b'1' + ic) as char;

                let index = ic * 8 + il;
                let case = Square::new(index).unwrap();

                assert_eq!(case.to_uci(), format!("{l}{c}"), "index {index}");
            }
        }
    }

        #[test]
    fn from_uci_of_known_squares() {
        let cases = [("a1", 0), ("h1", 7), ("e4", 28), ("a8", 56), ("h8", 63)];
        for (text, index) in cases {
            assert_eq!(
                Square::from_uci(text).map(|sq| sq.index()),
                Some(index),
                "texte {text:?}"
            );
        }
    }

    #[test]
    fn from_uci_rejects_wrong_length() {
        for text in ["", "a", "e44", "e4 ", " e4", "e2e4"] {
            assert!(Square::from_uci(text).is_none(), "texte {text:?}");
        }
    }

    #[test]
    fn from_uci_rejects_file_out_of_range() {
        // "`" est le caractère juste avant 'a', "i" celui juste après 'h'
        for text in ["`1", "i1", "z1", "A1", "H1"] {
            assert!(Square::from_uci(text).is_none(), "texte {text:?}");
        }
    }

    #[test]
    fn from_uci_rejects_rank_out_of_range() {
        // '0' est juste avant '1', '9' juste après '8'
        for text in ["a0", "a9", "aa", "a/"] {
            assert!(Square::from_uci(text).is_none(), "texte {text:?}");
        }
    }

    #[test]
    fn from_uci_rejects_swapped_order() {
        for text in ["1a", "8h", "11"] {
            assert!(Square::from_uci(text).is_none(), "texte {text:?}");
        }
    }

    #[test]
    fn from_uci_rejects_non_ascii_without_panicking() {
        // "é" fait 2 octets : il passe le test de longueur et doit échouer sur la lettre
        for text in ["é", "é1", "aé", "🙂"] {
            assert!(Square::from_uci(text).is_none(), "texte {text:?}");
        }
    }

    #[test]
    fn from_uci_inverts_to_uci_for_all_squares() {
        for i in 0..64u8 {
            let sq = Square::new(i).unwrap();
            assert_eq!(Square::from_uci(&sq.to_uci()), Some(sq), "index {i}");
        }
    }

    #[test]
    fn to_uci_inverts_from_uci_for_all_valid_texts() {
        for letter in b'a'..=b'h' {
            for digit in b'1'..=b'8' {
                let text = format!("{}{}", letter as char, digit as char);
                let sq = Square::from_uci(&text).unwrap();
                assert_eq!(sq.to_uci(), text);
            }
        }
    }
}
