use crate::position::Position;
use crate::position::NUM_PILES;
use crate::position::BOARD_SIZE;
use crate::stone::Stone;
use crate::stone_pile::StonePile;
use crate::game_state::Move;
use crate::stone_pile::PileTop;
use crate::game_state::PlayerColor;
use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct Board {
    piles: [StonePile; NUM_PILES],
}

impl Board {
    pub fn starting_board() -> Self {
        let mut b = Board::default();
        for pos in [Position::top_left(), Position::top_right(), Position::bottom_left(), Position::bottom_right()] {
            let p = &mut b.piles[pos.to_index()];
            p.add_stone(Stone::Neutral);
            p.add_stone(Stone::Neutral);
        }
        b
    }

    pub fn top(&self) -> BoardTop {
        let mut piles = [PileTop::Empty; NUM_PILES];
        for i in 0..NUM_PILES {
            piles[i] = self.piles[i].top();
        }
        BoardTop::new(piles)
    }

    pub fn iter_piles(&self) -> impl Iterator<Item = (Position, &StonePile)> {
        return self.piles.iter().enumerate().map(|(i,p)| (Position::from_index(i), p))
    }

    pub fn apply_move(&mut self, mv: &Move) {
        let mut hand = self.pile_mut(mv.path.start()).take_pile();
        if hand.len() == 0 {
            panic!("Cannot start move {mv:?} in empty pile")
        }
        hand.push(mv.new_stone);
        if hand.len() != mv.path.steps().len() {
            panic!("Path too short for move {mv:?} to handle hand {hand:?}")
        }
        for (i, pos) in mv.path.iter_positions().enumerate() {
            self.pile_mut(pos).add_stone(hand[i]);
        }
    }

    fn pile_mut(&mut self, pos: Position) -> &mut StonePile {
        return &mut self.piles[pos.to_index()]
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (pos, pile) in self.iter_piles() {
            pile.fmt(f)?;
            write!(f, "{}", if pos.next_on_new_row() { "\n" } else { " " })?;
        }
        Ok(())
    }
}


/// BoardTop is a representation of the top of the board without any detail about lower stones in stacks
#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash, Default)]
pub struct BoardTop([PileTop; NUM_PILES]);

impl BoardTop {
    fn new(piles: [PileTop; NUM_PILES]) -> Self {
        Self(piles)
    }

    pub fn iter_piles(&self) -> impl Iterator<Item = (Position, &PileTop)> {
        return self.0.iter().enumerate().map(|(i,p)| (Position::from_index(i), p))
    }

    pub fn winner(&self) -> Option<PlayerColor> {
        // Check rows
        'row_loop: for y in 0..BOARD_SIZE {
            let row_start = self.0[Position::from_coord(0,y).to_index()];
            let row_player = match PlayerColor::from_pile_top(row_start) {
                None => { continue 'row_loop; },
                Some(c) => c,
            };
            for x in 1..BOARD_SIZE {
                if self.0[Position::from_coord(x,y).to_index()] != row_start {
                    continue 'row_loop;
                }
            }
            return Some(row_player);
        }
        // Check cols
        'col_loop: for x in 0..BOARD_SIZE {
            let col_start = self.0[Position::from_coord(x,0).to_index()];
            let col_player = match PlayerColor::from_pile_top(col_start) {
                None => { continue 'col_loop; },
                Some(c) => c,
            };
            for y in 1..BOARD_SIZE {
                if self.0[Position::from_coord(x,y).to_index()] != col_start {
                    continue 'col_loop;
                }
            }
            return Some(col_player);
        }
        // Check diagonal 1, using icky goto-style break for consistency with rows/cols
        'diag_1: {
            let diag_start = self.0[Position::from_coord(0,0).to_index()];
            let diag_player = match PlayerColor::from_pile_top(diag_start) {
                None => { break 'diag_1; },
                Some(c) => c,
            };
            for i in 1..BOARD_SIZE {
                if self.0[Position::from_coord(i,i).to_index()] != diag_start {
                    break 'diag_1;
                }
            }
            return Some(diag_player);
        }
        // Check diagonal 2, using icky goto-style break for consistency with rows/cols
        'diag_2: {
            let diag_start = self.0[Position::from_coord(0,BOARD_SIZE-1).to_index()];
            let diag_player = match PlayerColor::from_pile_top(diag_start) {
                None => { break 'diag_2; },
                Some(c) => c,
            };
            for i in 1..BOARD_SIZE {
                if self.0[Position::from_coord(i,BOARD_SIZE-1-i).to_index()] != diag_start {
                    break 'diag_2;
                }
            }
            return Some(diag_player);
        }
        None
    }
}

impl fmt::Display for BoardTop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (pos, pile) in self.iter_piles() {
            pile.fmt(f)?;
            if pos.next_on_new_row() { write!(f, "\n")?; }
        }
        Ok(())
    }
}