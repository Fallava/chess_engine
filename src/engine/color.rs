#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn opposite(self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    pub fn index(self) -> usize {
        self as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opposite_of_white_is_black() {
        assert_eq!(Color::White.opposite(), Color::Black)
    }

    #[test]
    fn opposite_of_black_is_white() {
        assert_eq!(Color::Black.opposite(), Color::White)
    }

    #[test]
    fn opposite_twice_return_original() {
        for c in [Color::White, Color::Black] {
            assert_eq!(c.opposite().opposite(), c, "pour {c:?}");
        }
    }
}
