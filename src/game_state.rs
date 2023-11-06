use tinyvec::ArrayVec;

use crate::path::PathGenerator;
use crate::{stone::Stone, stone_pile::PileTop, path::Path, board::Board, position::{Position, NUM_PILES}};

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
#[repr(u8)]
pub enum PlayerColor {
    Red,
    White
}

impl PlayerColor {
    pub fn from_pile_top(pile_top: PileTop) -> Option<PlayerColor> {
        match pile_top {
            PileTop::Empty => None,
            PileTop::NeutralStone => None,
            PileTop::RedStone => Some(PlayerColor::Red),
            PileTop::WhiteStone => Some(PlayerColor::White),
        }
    }

    pub fn opponent(&self) -> PlayerColor {
        match self {
            PlayerColor::Red => PlayerColor::White,
            PlayerColor::White => PlayerColor::Red
        }
    }

    pub fn stone_color(&self) -> Stone {
        match self {
            PlayerColor::Red => Stone::Red,
            PlayerColor::White => Stone::White
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub struct Move {
    pub new_stone: Stone,
    pub path: Path,
}

pub struct MoveGenerator {
    piles: ArrayVec<[u8; NUM_PILES]>,
    start_index: usize,
    path_generator: PathGenerator,
}

impl MoveGenerator {
    // MoveGenerator iterates all the possible moves that can be made on a board.
    pub fn new(b: &Board) -> MoveGenerator {
        let mut piles = ArrayVec::<[u8; NUM_PILES]>::new();
        for (_, p) in b.iter_piles() {
            piles.push(p.height());
        }
        let mut start_index: usize = 0;
        for i in 0..NUM_PILES {
            if piles[i] > 0 {
                start_index = i;
                break;
            }
        }
        MoveGenerator {
            piles,
            start_index: start_index,
            path_generator: PathGenerator::new(Position::from_index(start_index), piles[start_index]+1),
        }
    }
}

impl Iterator for MoveGenerator {
    // We can refer to this type using Self::Item
    type Item = Path;

    fn next(&mut self) -> Option<Self::Item> {
        let mut path = self.path_generator.next();
        while path == None && self.start_index < NUM_PILES-1 {
            self.start_index += 1;
            if self.piles[self.start_index] == 0 {
                continue;
            }
            self.path_generator = PathGenerator::new(Position::from_index(self.start_index), self.piles[self.start_index]+1);
            path = self.path_generator.next();
        }
        path
    }
}