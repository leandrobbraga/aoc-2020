use crate::Direction::{East, Forward, Left, North, Right, South, West};
use std::fs;
use std::ops::Add;

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    value: u32,
}

#[derive(Debug)]
struct Position {
    position: (i32, i32),
    direction: Direction,
}

impl Add for Position {
    type Output = Position;

    fn add(self, other: Position) -> Self::Output {
        let new_position = (
            self.position.0 + other.position.0,
            self.position.1 + other.position.1,
        );

        Position {
            position: new_position,
            direction: other.direction,
        }
    }
}

fn main() {
    let contents: String = fs::read_to_string("./examples/input/day-12.txt").unwrap();
    let instructions = parse_input(contents);
    solve_part_01(&instructions);
}

fn parse_input(input: String) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| Instruction {
            direction: match &line[0..1] {
                "N" => North,
                "S" => South,
                "E" => East,
                "W" => West,
                "L" => Left,
                "R" => Right,
                "F" => Forward,
                _ => panic!("Wrong input direction!"),
            },
            value: line[1..].parse().unwrap(),
        })
        .collect()
}

fn solve_part_01(instructions: &Vec<Instruction>) {
    let mut ship: Position = Position {
        position: (0, 0),
        direction: East,
    };

    for instruction in instructions {
        println!("{:?}", ship);
        println!("{:?}", instruction);
        let new_position = make_position(instruction, ship.direction);
        ship = ship + new_position;
        println!("{:?}", ship);
    }

    println!("{} {}", ship.position.0, ship.position.1);
    println!(
        "Part 1 - {}",
        (ship.position.0.abs() + ship.position.1.abs())
    );
}

fn make_position(instruction: &Instruction, position_direction: Direction) -> Position {
    match instruction.direction {
        North => Position {
            position: (instruction.value as i32, 0),
            direction: position_direction,
        },
        South => Position {
            position: (-(instruction.value as i32), 0),
            direction: position_direction,
        },
        East => Position {
            position: (0, instruction.value as i32),
            direction: position_direction,
        },
        West => Position {
            position: (0, -(instruction.value as i32)),
            direction: position_direction,
        },
        Forward => make_position(
            &Instruction {
                direction: position_direction,
                value: instruction.value,
            },
            position_direction,
        ),
        _ => {
            let mut direction = position_direction;
            for _i in 0..instruction.value / 90 {
                direction = rotate_direction(direction, instruction.direction);
            }
            Position {
                position: (0, 0),
                direction,
            }
        }
    }
}

fn rotate_direction(position_direction: Direction, instruction_direction: Direction) -> Direction {
    match position_direction {
        North => match instruction_direction {
            Left => West,
            Right => East,
            _ => panic!("Instruction direction should be always relative!"),
        },
        South => match instruction_direction {
            Left => East,
            Right => West,
            _ => panic!("Instruction direction should be always relative!"),
        },
        East => match instruction_direction {
            Left => North,
            Right => South,
            _ => {
                println!("{:?}", instruction_direction);
                panic!("Instruction direction should be always relative!")
            }
        },
        West => match instruction_direction {
            Left => South,
            Right => North,
            _ => panic!("Instruction direction should be always relative!"),
        },
        _ => panic!("Position direction should be always absolute!"),
    }
}
