use super::bitboard::Bitboard;
use super::color::Color;
use super::square::Square;

pub fn king_attacks(sq: Square) -> Bitboard {
    let b = Bitboard::from_square(sq);
    b.north() | b.south() | b.east() | b.west() | b.north_east() | b.north_west() | b.south_east() | b.south_west()
}

pub fn knight_attacks(sq: Square) -> Bitboard {
    let b = Bitboard::from_square(sq);
    b.north().north_east() | b.north().north_west() 
    | b.east().north_east() | b.west().north_west()
    | b.south().south_east() | b.south().south_west() 
    | b.east().south_east() | b.west().south_west()
}

pub fn pawn_attacks(color: Color, sq: Square) -> Bitboard {
    let b = Bitboard::from_square(sq);
    match color {
        Color::White => {
            b.north_east() | b.north_west()
        },
        Color::Black => {
            b.south_east() | b.south_west()
        }
    }
}

pub fn rook_attacks(sq: Square, occupied: Bitboard) -> Bitboard {
    let s1 = slide(sq, occupied, Bitboard::north);
    let s2 = slide(sq, occupied, Bitboard::south);
    let s3 = slide(sq, occupied, Bitboard::east);
    let s4 = slide(sq, occupied, Bitboard::west);
    s1 | s2 | s3 | s4
}

pub fn bishop_attacks(sq: Square, occupied: Bitboard) -> Bitboard{
    let s1 = slide(sq, occupied, Bitboard::north_east);
    let s2 = slide(sq, occupied, Bitboard::north_west);
    let s3 = slide(sq, occupied, Bitboard::south_east);
    let s4 = slide(sq, occupied, Bitboard::south_west);
    s1 | s2 | s3 | s4
}

pub fn queen_attacks(sq: Square, occupied: Bitboard) -> Bitboard {
    rook_attacks(sq, occupied) | bishop_attacks(sq, occupied)
}

fn slide(sq: Square, occupied: Bitboard, step: fn(Bitboard) -> Bitboard) -> Bitboard {
    let mut result = Bitboard::EMPTY;
    let mut ray = Bitboard::from_square(sq);
    loop {
        ray = step(ray);
        if ray.is_empty() {break}
        result |= ray;
        if !(ray & occupied).is_empty() {break}
    }
    result
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn king_attacks_count_by_position() {
        assert_eq!(king_attacks(Square::new(28).unwrap()).count(), 8); // e4
        assert_eq!(king_attacks(Square::new(0).unwrap()).count(), 3);  // a1
        assert_eq!(king_attacks(Square::new(4).unwrap()).count(), 5);  // e1
        assert_eq!(king_attacks(Square::new(0).unwrap()), Bitboard::new(0x302)); // a1 : b1, a2, b2
    }

    #[test]
    fn knight_attacks_known_squares() {
        assert_eq!(knight_attacks(Square::new(28).unwrap()).count(), 8); // e4 : le milieu
        assert_eq!(knight_attacks(Square::new(4).unwrap()).count(), 4);  // e1 : le bord
        // a1 : seulement b3 (index 17) et c2 (index 10)
        assert_eq!(knight_attacks(Square::new(0).unwrap()), Bitboard::new((1 << 17) | (1 << 10)));
    }

    #[test]
    fn pawn_attacks_by_color_and_edge() {
        assert_eq!(pawn_attacks(Color::White, Square::new(28).unwrap()).count(), 2); // e4 : d5, f5
        assert_eq!(pawn_attacks(Color::White, Square::new(8).unwrap()).count(), 1);  // a2 : seulement b3
        assert_eq!(pawn_attacks(Color::Black, Square::new(36).unwrap()).count(), 2); // e5 : d4, f4
    }

    #[test]
    fn sliding_attacks_known_positions() {
        let a1 = Square::new(0).unwrap();
        let e4 = Square::new(28).unwrap();

        // plateau vide
        assert_eq!(rook_attacks(a1, Bitboard::EMPTY).count(), 14);
        assert_eq!(bishop_attacks(e4, Bitboard::EMPTY).count(), 13);
        assert_eq!(queen_attacks(e4, Bitboard::EMPTY).count(), 27);

        // un bloqueur en a4 (index 24) : la tour voit a2, a3, a4 mais pas a5..a8
        let blocker = Bitboard::from_square(Square::new(24).unwrap());
        let attacks = rook_attacks(a1, blocker);
        assert_eq!(attacks.count(), 10);               // 3 vers le haut + 7 vers la droite
        assert!(attacks.contains(Square::new(24).unwrap()));   // le bloqueur est inclus
        assert!(!attacks.contains(Square::new(32).unwrap()));  // a5 est masquée
    }
}