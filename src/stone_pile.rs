use crate::stone::Stone;
use tinyvec::ArrayVec;
use std::fmt;

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash, Default)]
pub struct StonePile(ArrayVec<[Stone; PILE_CAPACITY]>);

pub const PILE_CAPACITY: usize = 8;

impl StonePile {
    pub fn top(&self) -> PileTop {
        match self.0.last() {
            None => PileTop::Empty,
            Some(Stone::Red) => PileTop::RedStone,
            Some(Stone::White) => PileTop::WhiteStone,
            Some(Stone::Neutral) => PileTop::NeutralStone,
        }
    }

    pub fn height(&self) -> u8 {
        self.0.len() as u8
    }

    pub fn add_stone(&mut self, s: Stone) {
        self.0.push(s)
    }
    
    pub fn take_pile(&mut self) -> ArrayVec<[Stone; PILE_CAPACITY]> {
        let mut pile = ArrayVec::<[Stone; PILE_CAPACITY]>::new();
        (self.0, pile) = (pile, self.0);
        pile
    }
}

impl fmt::Display for StonePile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0.is_empty() {
            return write!(f, "[{:.<PILE_CAPACITY$}]", PileTop::Empty.to_string())
        }
        let stack = self.0.iter().fold(String::new(), |acc, &arg| acc + &arg.to_string());
        write!(f, "[{:.<PILE_CAPACITY$}]", stack)
    }
}


#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash, Default)]
#[repr(u8)]
pub enum PileTop {
    #[default]
    Empty,
    RedStone,
    WhiteStone,
    NeutralStone,
}

impl fmt::Display for PileTop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PileTop::Empty => write!(f, "."),
            PileTop::RedStone => write!(f, "R"),
            PileTop::WhiteStone => write!(f, "W"),
            PileTop::NeutralStone => write!(f, "n"),
        }
    }
}