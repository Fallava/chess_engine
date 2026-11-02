use super::Board;

impl Board {
    pub fn _perft(&self, depth: u32) -> u64 {
        if depth == 0 {
            return 1;
        }
        let moves = self.legal_moves();
        if depth == 1 {
            return moves.len() as u64; // inutile de jouer le dernier coup
        }
        moves
            .iter()
            .map(|&mv| {
                let mut next = self.clone();
                next.make_move(mv);
                next._perft(depth - 1)
            })
            .sum()
    }

    /// Le perft de chaque coup de la racine : sert à localiser un bug.
    pub fn _divide(&self, depth: u32) -> Vec<(String, u64)> {
        let mut result: Vec<(String, u64)> = self
            .legal_moves()
            .iter()
            .map(|&mv| {
                let mut next = self.clone();
                next.make_move(mv);
                (mv.to_uci(), next._perft(depth - 1))
            })
            .collect();
        result.sort();
        result
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const CASES: &[(&str, &str, &[u64])] = &[
        ("startpos", "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            &[20, 400, 8_902, 197_281, 4_865_609]),
        ("kiwipete", "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
            &[48, 2_039, 97_862, 4_085_603]),
        ("pos3", "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1",
            &[14, 191, 2_812, 43_238, 674_624]),
        ("pos4", "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
            &[6, 264, 9_467, 422_333]),
        ("pos5", "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
            &[44, 1_486, 62_379, 2_103_487]),
        ("pos6", "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
            &[46, 2_079, 89_890, 3_894_594]),
    ];

    #[ignore]
    #[test]
    fn perft_reference_positions() {
        for (name, fen, expected) in CASES {
            let board = Board::from_fen(fen).unwrap();
            for (i, &exp) in expected.iter().enumerate() {
                let depth = (i + 1) as u32;
                assert_eq!(board._perft(depth), exp, "{name} à la profondeur {depth}");
            }
        }
    }
    }