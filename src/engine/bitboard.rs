use std::ops::{
    BitOr, BitOrAssign,
    BitAnd, BitAndAssign,
    BitXor, BitXorAssign,
    Not
};

use super::square::Square;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Bitboard(u64);

impl Bitboard {
    pub const EMPTY: Bitboard = Bitboard(0);

    pub const FILE_A: Bitboard = Bitboard(0x0101_0101_0101_0101);
    pub const FILE_B: Bitboard = Bitboard(0x0202_0202_0202_0202);
    pub const FILE_C: Bitboard = Bitboard(0x0404_0404_0404_0404);
    pub const FILE_D: Bitboard = Bitboard(0x0808_0808_0808_0808);
    pub const FILE_E: Bitboard = Bitboard(0x1010_1010_1010_1010);
    pub const FILE_F: Bitboard = Bitboard(0x2020_2020_2020_2020);
    pub const FILE_G: Bitboard = Bitboard(0x4040_4040_4040_4040);
    pub const FILE_H: Bitboard = Bitboard(0x8080_8080_8080_8080);

    pub const RANK_1: Bitboard = Bitboard(0x0000_0000_0000_00FF);
    pub const RANK_2: Bitboard = Bitboard(0x0000_0000_0000_FF00);
    pub const RANK_3: Bitboard = Bitboard(0x0000_0000_00FF_0000);
    pub const RANK_4: Bitboard = Bitboard(0x0000_0000_FF00_0000);
    pub const RANK_5: Bitboard = Bitboard(0x0000_00FF_0000_0000);
    pub const RANK_6: Bitboard = Bitboard(0x0000_FF00_0000_0000);
    pub const RANK_7: Bitboard = Bitboard(0x00FF_0000_0000_0000);
    pub const RANK_8: Bitboard = Bitboard(0xFF00_0000_0000_0000);

    pub const fn new(bits: u64) -> Bitboard { Bitboard(bits) }
    pub const fn bits(self) -> u64 { self.0 }

    pub fn from_square(sq: Square) -> Self {
        Bitboard(1 << sq.index())
    }

    pub fn contains(self, sq: Square) -> bool {
        (self.0 >> sq.index()) & 1 == 1
    }

    pub fn insert(&mut self, sq: Square) {
        let mask = Self::from_square(sq);
        self.0 |= mask.0;
    }

    pub fn remove(&mut self, sq: Square) {
        let mask = Self::from_square(sq);
        self.0 &= !mask.0;
    }

    pub fn count(self) -> u32 {
        self.0.count_ones()
    }

    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub fn north(self) -> Bitboard {
        Bitboard(self.0 << 8)
    }

    pub fn south(self) -> Bitboard {
        Bitboard(self.0 >> 8)
    }

    pub fn east(self) -> Bitboard {
        Bitboard(self.0 << 1 & !Self::FILE_A.0)
    }

    pub fn west(self) -> Bitboard {
        Bitboard(self.0 >> 1 & !Self::FILE_H.0)
    }

    pub fn north_east(self) -> Bitboard {
        Bitboard(self.0 << 9 & !Self::FILE_A.0)
    }

    pub fn north_west(self) -> Bitboard {
        Bitboard(self.0 << 7 & !Self::FILE_H.0)
    }

    pub fn south_east(self) -> Bitboard {
        Bitboard(self.0 >> 7 & !Self::FILE_A.0)
    }

    pub fn south_west(self) -> Bitboard {
        Bitboard(self.0 >> 9 & !Self::FILE_H.0)
    }

    pub fn lsb(self) -> Option<Square> {
        let index = self.0.trailing_zeros() as u8;
        Square::new(index)
    }

    pub fn pop_lsb(&mut self) -> Option<Square> {
        let sq = self.lsb()?;
        self.0 &= self.0 - 1;
        Some(sq)
    } 
}

impl Iterator for Bitboard {
    type Item = Square;

    fn next(&mut self) -> Option<Square> {
        self.pop_lsb()
    }
}

impl BitOr for Bitboard {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0
    }
}

impl BitAnd for Bitboard {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0
    }
}

impl BitXor for Bitboard {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0
    }
}

impl Not for Bitboard {
    type Output = Self;
    fn not(self) -> Self::Output {
        Bitboard(!self.0)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_contains_no_square() {
        for i in 0..64 {
            let sq = Square::new(i).unwrap();
            assert!(!Bitboard::EMPTY.contains(sq), "case {i}");
        }
    }
    
    #[test]
    fn bitboard_1_contains_square_0() {
        let bb = Bitboard(1);
        let sq = Square::new(0).unwrap();
        assert!(bb.contains(sq))
    }

    #[test]
    fn bitboard_2_contains_square_1() {
        let bb = Bitboard(2);
        let sq = Square::new(1).unwrap();
        assert!(bb.contains(sq))
    }

    #[test]
    fn bitboard_0xff00_contains_exactly_rank_2() {
        let bb = Bitboard(0xFF00);
        for i in 0..64 {
            let sq = Square::new(i).unwrap();
            assert_eq!(bb.contains(sq), sq.rank() == 1, "case {i}");
        }
    }

        #[test]
    fn insert_sets_only_that_bit() {
        let mut bb = Bitboard::EMPTY;
        bb.insert(Square::new(28).unwrap());   // e4
        assert_eq!(bb, Bitboard(1 << 28));
    }

    #[test]
    fn insert_twice_keeps_the_square() {
        let mut bb = Bitboard::EMPTY;
        let sq = Square::new(28).unwrap();
        bb.insert(sq);
        bb.insert(sq);
        assert_eq!(bb, Bitboard(1 << 28));
    }

    #[test]
    fn insert_keeps_existing_squares_and_handles_corners() {
        let mut bb = Bitboard::EMPTY;
        bb.insert(Square::new(0).unwrap());    // a1
        bb.insert(Square::new(63).unwrap());   // h8
        assert_eq!(bb, Bitboard(1 | (1 << 63)));
    }

    #[test]
    fn remove_e4_from_bitboard_result_empty() {
        let mut bb = Bitboard::EMPTY;
        let sq = Square::new(28).unwrap();
        bb.insert(sq);
        bb.remove(sq);

        assert_eq!(bb, Bitboard::EMPTY);
    }

    #[test]
    fn count_8_on_bitboard() {
        assert!(Bitboard(0xFF00).count() == 8)
    }

    #[test]
    fn is_empty_on_empty() {
        assert!(Bitboard::EMPTY.is_empty());
        assert!(!Bitboard(10).is_empty());
    }

        #[test]
    fn from_square_sets_only_that_bit() {
        assert_eq!(Bitboard::from_square(Square::new(28).unwrap()), Bitboard(1 << 28)); // e4
        assert_eq!(Bitboard::from_square(Square::new(63).unwrap()), Bitboard(1 << 63)); // h8
    }

    #[test]
    fn remove_absent_square_changes_nothing() {
        let mut bb = Bitboard(0xFF00);
        bb.remove(Square::new(28).unwrap());
        assert_eq!(bb, Bitboard(0xFF00));
    }

    #[test]
    fn files_contain_exactly_their_squares() {
        let files = [
            Bitboard::FILE_A, Bitboard::FILE_B, Bitboard::FILE_C, Bitboard::FILE_D,
            Bitboard::FILE_E, Bitboard::FILE_F, Bitboard::FILE_G, Bitboard::FILE_H,
        ];
        for (f, bb) in files.iter().enumerate() {
            for i in 0..64 {
                let sq = Square::new(i).unwrap();
                assert_eq!(bb.contains(sq), sq.file() as usize == f, "colonne {f}, case {i}");
            }
        }
    }

    #[test]
    fn ranks_contain_exactly_their_squares() {
        let ranks = [
            Bitboard::RANK_1, Bitboard::RANK_2, Bitboard::RANK_3, Bitboard::RANK_4,
            Bitboard::RANK_5, Bitboard::RANK_6, Bitboard::RANK_7, Bitboard::RANK_8,
        ];
        for (r, bb) in ranks.iter().enumerate() {
            for i in 0..64 {
                let sq = Square::new(i).unwrap();
                assert_eq!(bb.contains(sq), sq.rank() as usize == r, "rangée {r}, case {i}");
            }
        }
    }

    #[test]
    fn operators_behave_as_set_operations() {
        let a = Bitboard::RANK_1;
        let b = Bitboard::FILE_A;
        assert_eq!(a | b, Bitboard(0x0101_0101_0101_01FF));
        assert_eq!(a & b, Bitboard(1));
        assert_eq!(a ^ b, Bitboard(0x0101_0101_0101_01FE));
        assert_eq!(!Bitboard::EMPTY, Bitboard(u64::MAX));

        let (mut or, mut and, mut xor) = (a, a, a);
        or |= b;
        and &= b;
        xor ^= b;
        assert_eq!((or, and, xor), (a | b, a & b, a ^ b));
    }

        #[test]
    fn shifts_move_one_square_and_drop_off_the_board() {
        let e4 = Bitboard::from_square(Square::new(28).unwrap());
        let sq = |i| Bitboard::from_square(Square::new(i).unwrap());

        assert_eq!(e4.north(), sq(36));       // e5
        assert_eq!(e4.south(), sq(20));       // e3
        assert_eq!(e4.east(), sq(29));        // f4
        assert_eq!(e4.west(), sq(27));        // d4
        assert_eq!(e4.north_east(), sq(37));  // f5
        assert_eq!(e4.north_west(), sq(35));  // d5
        assert_eq!(e4.south_east(), sq(21));  // f3
        assert_eq!(e4.south_west(), sq(19));  // d3

        // débordements : la pièce sort du plateau, elle ne réapparaît pas de l'autre côté
        assert_eq!(Bitboard::FILE_H.east(), Bitboard::EMPTY);
        assert_eq!(Bitboard::FILE_A.west(), Bitboard::EMPTY);
        assert_eq!(Bitboard::FILE_H.north_east(), Bitboard::EMPTY);
        assert_eq!(Bitboard::FILE_A.north_west(), Bitboard::EMPTY);
        assert_eq!(Bitboard::FILE_H.south_east(), Bitboard::EMPTY);
        assert_eq!(Bitboard::FILE_A.south_west(), Bitboard::EMPTY);
        assert_eq!(Bitboard::RANK_8.north(), Bitboard::EMPTY);
        assert_eq!(Bitboard::RANK_1.south(), Bitboard::EMPTY);
    }

    #[test]
    fn iterating_yields_squares_in_increasing_order() {
        let squares: Vec<usize> = Bitboard(0b1010_0001).map(|sq| sq.index()).collect();
        assert_eq!(squares, vec![0, 5, 7]);
        assert_eq!(Bitboard::EMPTY.count(), 0);

        let mut empty = Bitboard::EMPTY;
        assert_eq!(empty.next(), None);
    }
}