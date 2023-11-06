use crate::board::{Board, BoardTop};
use crate::game_state::{Move, MoveGenerator, PlayerColor};
use crate::path::{Path, PathGenerator};
use crate::stone::Stone;
use crate::stone_pile::{StonePile, PILE_CAPACITY};
use std::time::SystemTime;
use tinyvec::array_vec;

mod board;
mod game_state;
mod path;
mod position;
mod stone;
mod stone_pile;

fn main() {
    let moves = 4;
    let now = SystemTime::now();

    let b = Board::starting_board();
    let p = PlayerColor::White;
    let mut res = DfsResults::default();
    dfs(b, p, moves, &mut res);

    println!(
        "Took {}sec to evaluate {:?}",
        now.elapsed().expect("should get time").as_secs(),
        res
    );
}

#[derive(Debug, Default)]
struct DfsResults {
    count: i32,
    red_wins: i32,
    white_wins: i32,
}

fn dfs(board: Board, player: PlayerColor, moves: i32, res: &mut DfsResults) {
    if moves == 0 {
        res.count += 1;
        match board.top().winner() {
            Some(PlayerColor::Red) => res.red_wins += 1,
            Some(PlayerColor::White) => res.white_wins += 1,
            _ => (),
        }
        return;
    }
    let it = MoveGenerator::new(&board);
    for p in it {
        let mut new_board = board;
        new_board.apply_move(&Move {
            new_stone: player.stone_color(),
            path: p,
        });
        dfs(new_board, player.opponent(), moves - 1, res)
    }
}
