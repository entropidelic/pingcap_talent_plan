use serde::{Deserialize, Serialize};
use std::{fs, io};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Move {
    dir: Direction,
    steps: usize,
}

impl Move {
    fn new(dir: Direction, steps: usize) -> Move {
        Move { dir, steps }
    }
}

fn main() -> Result<(), io::Error> {
    let move_a = Move::new(Direction::Left, 5);
    let json_writer = fs::File::create("move.json")?;
    serde_json::to_writer(&json_writer, &move_a)?;

    let json_reader = fs::File::open("move.json")?;
    let move_b: Move = serde_json::from_reader(&json_reader)?;

    println!("DESERIALIZED MOVE: {:?}", move_b);
    assert_eq!(move_a, move_b);

    Ok(())
}
