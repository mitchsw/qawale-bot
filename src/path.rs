use crate::position::NUM_PILES;
use crate::position::Position;
use crate::position::Direction;
use crate::stone_pile::PILE_CAPACITY;
use tinyvec::ArrayVec;
use std::fmt;

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub struct Path {
    start: Position,
    steps: ArrayVec<[Direction; PILE_CAPACITY]>,
    end: Position,
}

impl Path {
    pub fn new(start: Position, steps: ArrayVec<[Direction; PILE_CAPACITY]>) -> Self {
        let mut end = start;
        for dir in steps {
            end = end.step(dir);
        }
        Self{start, steps, end}
    }

    pub fn new_empty(start: Position) -> Self {
        Self{start, steps: ArrayVec::<[Direction; PILE_CAPACITY]>::new(), end: start}
    }

    pub fn start(&self) -> Position {
        self.start
    }

    pub fn steps(&self) -> &ArrayVec<[Direction; PILE_CAPACITY]> {
        &self.steps
    }

    pub fn end(&self) -> Position {
        self.end
    }

    pub fn can_step(&self, dir: Direction) -> bool {
        self.end.can_step(dir)
    }

    pub fn step(&mut self, dir: Direction) {
        self.end = self.end.step(dir);
        self.steps.push(dir);
    }

    pub fn pop_step(&mut self) -> Direction {
        let dir = self.steps.pop().expect("expect step to pop");
        self.end = self.end.step(dir.reverse());
        dir
    }

    pub fn len(&self) -> usize {
        self.steps.len()
    }

    // This iterator will output len() positions, not including the start position.
    pub fn iter_positions(&self) -> impl Iterator<Item = Position> + '_ {
        PathPosIterator{path: &self, pos: self.start, step: 0}
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.len() == 0 {
            return write!(f, "[empty-path]");
        }
        let mut rep = vec![String::from("."); NUM_PILES];
        //rep[self.start.to_index()] = String::from("S");
        let mut pos = self.start;
        for dir in self.steps {
            rep[pos.to_index()] = dir.to_string();
            pos = pos.step(dir);
        }
        rep[self.start.to_index()] = match self.steps[0] {
            Direction::Down => "↧",
            Direction::Left => "↤",
            Direction::Right => "↦",
            Direction::Up => "↥",
        }.to_string();
        rep[self.end.to_index()] = String::from("x");
        for (i, p) in rep.iter().enumerate() {
            write!(f, "{}", p)?;
            if Position::from_index(i).next_on_new_row() {
                write!(f, "\n")?;
            }
        }
        write!(f, "\n")?;

        self.start.fmt(f)?;
        for (i, pos) in self.iter_positions().enumerate() {
            self.steps[i].fmt(f)?;
            pos.fmt(f)?;
        }
        Ok(())
    }
}

pub struct PathPosIterator<'a> {
    path: &'a Path,
    pos: Position,
    step: usize,
}

impl Iterator for PathPosIterator<'_> {
    // We can refer to this type using Self::Item
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        if self.step == self.path.len() {
            return None;
        }
        self.pos = self.pos.step(self.path.steps[self.step]);
        self.step += 1;
        Some(self.pos)
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub struct PathGenerator {
    length: usize,
    path: Path,
}

impl PathGenerator {
    // PathGenerator iterates all the possible paths of a specified length starting at a specified position.
    // Respects the confines of the board and rules of game (no u-turns).
    pub fn new(start: Position, length: u8) -> PathGenerator {
        PathGenerator{
            length: length as usize,
            path: Path::new_empty(start),
        }
    }

    fn complete_path(&mut self) {
        while self.path.len() < self.length {
            for dir in enum_iterator::all::<Direction>() {
                if !self.path.can_step(dir) {
                    continue;
                }
                if self.path.len() > 0 && dir.reverse() == *self.path.steps.last().unwrap() {
                    continue; // Can't do a u-turn.
                }
                self.path.step(dir);
                break;
            }
        }
    }
}

impl Iterator for PathGenerator {
    // We can refer to this type using Self::Item
    type Item = Path;

    fn next(&mut self) -> Option<Self::Item> {
        if self.length == 0 {
            return None; // An empty path generator
        }
        if self.path.len() == 0 {
            // First call to next, build the first path:
            self.complete_path();
            return Some(self.path);
        }

        let mut last_dir = self.path.pop_step();
        loop {
            let dir = enum_iterator::next(&last_dir);
            if dir == None { // Tried all directions at this step, pop back up to prior step.
                if self.path.len() == 0 {
                    return None;
                }
                last_dir = self.path.pop_step();
                continue;
            }
            let dir = dir.unwrap();
            if !self.path.can_step(dir) {
                last_dir = dir;
                continue;
            }
            if self.path.len() > 0 && dir.reverse() == *self.path.steps.last().unwrap() {
                last_dir = dir;
                continue; // Can't do a u-turn.
            }

            self.path.step(dir);
            if self.path.len() != self.length {
                // If we popped one or more steps, complete the remaining required steps.
                self.complete_path();
            }
            return Some(self.path);
        }
    }
}
