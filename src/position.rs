
use std::fmt;

use enum_iterator::Sequence;

pub const BOARD_SIZE: u8 = 4;
pub const NUM_PILES: usize = (BOARD_SIZE*BOARD_SIZE) as usize;

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash, Default)]
pub struct Position(u8);

impl Position {
    pub fn from_index(i: usize) -> Position {
        Position(i as u8)
    }

    pub fn from_coord(x: u8, y: u8) -> Position {
        Position(x + y*(BOARD_SIZE))
    }

    pub fn top_left() -> Position {
        Position::from_coord(0, 0)
    }

    pub fn top_right() -> Position {
        Position::from_coord(0, BOARD_SIZE-1)
    }

    pub fn bottom_left() -> Position {
        Position::from_coord(BOARD_SIZE-1, 0)
    }

    pub fn bottom_right() -> Position {
        Position::from_coord(BOARD_SIZE-1, BOARD_SIZE-1)
    }
    
    pub fn x(&self) -> u8 {
        self.0 % BOARD_SIZE
    }

    pub fn y(&self) -> u8 {
        self.0 / BOARD_SIZE
    }

    pub fn to_index(&self) -> usize {
        self.0 as usize
    }

    // Returns true if the next position index (if there is one) is on the following row
    pub fn next_on_new_row(&self) -> bool {
        (self.0 + 1)%BOARD_SIZE == 0 && (self.0 + 1) < (NUM_PILES as u8)
    }

    pub fn can_step(&self, dir: Direction) -> bool {
        match dir {
            Direction::Right => self.x() < BOARD_SIZE-1,
            Direction::Down => self.y() < BOARD_SIZE-1,
            Direction::Left => self.x() > 0,
            Direction::Up => self.y() > 0,
        }
    }

    pub fn step(&self, dir: Direction) -> Position {
        match dir {
            Direction::Right => {
                if self.x() == BOARD_SIZE-1 { panic!("Trying to step {dir} from {self}"); }
                Position::from_coord(self.x()+1, self.y())
            }
            Direction::Down => {
                if self.y() == BOARD_SIZE-1 { panic!("Trying to step {dir} from {self}"); }
                Position::from_coord(self.x(), self.y()+1)
            }
            Direction::Left => {
                if self.x() == 0 { panic!("Trying to step {dir} from {self}"); }
                Position::from_coord(self.x()-1, self.y())
            }
            Direction::Up => {
                if self.y() == 0 { panic!("Trying to step {dir} from {self}"); }
                Position::from_coord(self.x(), self.y()-1)
            }
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x(), self.y())
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash, Default, Sequence)]
#[repr(u8)]
pub enum Direction {
    #[default]
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    pub fn reverse(&self) -> Direction {
        match self {
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Up => Direction::Down,
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Direction::Right => write!(f, "→"),
            Direction::Down => write!(f, "↓"),
            Direction::Left => write!(f, "←"),
            Direction::Up => write!(f, "↑"),
        }
    }
}