use crate::stone::Stone;
use crate::stone_pile::{StonePile, PILE_CAPACITY};
use crate::board::{Board, BoardTop};
use crate::position::{Position, Direction};
use crate::path::{PathGenerator, Path};
use crate::game_state::{MoveGenerator, Move};
use core::mem;
use tinyvec::array_vec;

mod stone;
mod board;
mod stone_pile;
mod position;
mod path;
mod game_state;

fn main() {
    println!("### MEMORY USAGE ####");
    println!("Stone size: {}b", mem::size_of::<Stone>());
    println!("Stone pile size: {}b", mem::size_of::<StonePile>());
    println!("Board size: {}b", mem::size_of::<Board>());
    println!("Board top size: {}b", mem::size_of::<BoardTop>());
    println!("Position size: {}b", mem::size_of::<Position>());
    println!("Move size: {}b", mem::size_of::<Move>());

    println!("\n\n### STARTING BOARD ####");
    let mut b = Board::starting_board();
    println!("{b}");
    println!("## Top:\n{}", b.top());
    println!("## Winner: {:?}", b.top().winner());
    println!("## Debug:\n{b:?}");

    println!("\n\n### APPLYING TEST MOVE ####");
    b.apply_move(&Move {
        new_stone: Stone::Red, 
        path: Path::new(
            Position::from_coord(0, 0),
            array_vec!([Direction; PILE_CAPACITY] => Direction::Right, Direction::Right, Direction::Right),
        ),
    });
    let mut b2 = b;
    b2.apply_move(&Move {
        new_stone: Stone::Orange, 
        path: Path::new(
            Position::from_coord(2, 0),
            array_vec!([Direction; PILE_CAPACITY] => Direction::Right, Direction::Down),
        ),
    });
    println!("{b}\n");
    println!("{b2}");


    println!("\n\n### TESTING PATH Generator ####");
    println!("## Top left:");
    let it = PathGenerator::new(Position::top_left(), 3);
    let b = Board::starting_board();
    for (i, p) in it.enumerate() {
        println!("Path {i}:\n{p}");
        let mut b2 = b;
        b2.apply_move(&Move { new_stone: Stone::Red, path: p });
        println!("{}\n", b2);
    }
    println!("## Some mid point:");
    let it = PathGenerator::new(Position::from_coord(1,2), 0);
    for (i, p) in it.enumerate() {
        println!("Path {i}:\n{p}\n");
    }

    println!("\n\n### TESTING MOVE Generator ####");
    println!("## Top left:");
    let b = Board::starting_board();
    let it = MoveGenerator::new(&b);
    for (i, p) in it.enumerate() {
        println!("Path {i}:\n{p}");
        let mut b2 = b;
        b2.apply_move(&Move { new_stone: Stone::Red, path: p });
        println!("{}\n", b2);
    }

    println!("\n\n### TESTING 3-step move Generator ####");
    let b = Board::starting_board();
    let it = MoveGenerator::new(&b);
    let mut i = 0;
    for p in it {
        let mut b2 = b;
        b2.apply_move(&Move { new_stone: Stone::Red, path: p });
        let it2 = MoveGenerator::new(&b2);
        for p2 in it2 {
            let mut b3 = b2;
            b3.apply_move(&Move { new_stone: Stone::Orange, path: p2 });
            let it3 = MoveGenerator::new(&b3);
            for p3 in it3 {
                let mut b4 = b3;
                b4.apply_move(&Move { new_stone: Stone::Red, path: p3 });
                println!("Opt {i}:\n{b4}\n");
                i += 1;
            }
        }
    }
}