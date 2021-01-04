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
    solve_part_02(&instructions);
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

fn solve_part_01(instructions: &[Instruction]) {
    let mut ship: Position = Position {
        position: (0, 0),
        direction: East,
    };

    for instruction in instructions {
        let new_position = make_position(instruction, ship.direction);
        ship = ship + new_position;
    }

    println!("Part 1");
    println!("------");
    println!(
        "Final Position - x:{} y:{}",
        ship.position.1, ship.position.0
    );
    println!(
        "Result - {}",
        (ship.position.0.abs() + ship.position.1.abs())
    );
}

fn solve_part_02(instructions: &[Instruction]) {
    let mut wp: Position = Position {
        position: (10, 1),
        direction: East,
    };

    let mut ship: Position = Position {
        position: (0, 0),
        direction: East,
    };

    for instruction in instructions {
        match instruction.direction {
            North | South | East | West => {
                wp = wp + make_position(instruction, ship.direction);
            }
            Left => wp = rotate_waypoint(wp, instruction.value as i32),
            Right => wp = rotate_waypoint(wp, -(instruction.value as i32)),
            Forward => {
                ship = ship
                    + Position {
                        position: (
                            wp.position.0 * instruction.value as i32,
                            wp.position.1 * instruction.value as i32,
                        ),
                        direction: Direction::North,
                    };
            }
        }
    }

    println!("Part 2");
    println!("------");
    println!(
        "Final Position - x:{} y:{}",
        ship.position.1, ship.position.0
    );
    println!(
        "Result - {}",
        (ship.position.0.abs() + ship.position.1.abs())
    );
}

fn rotate_waypoint(position: Position, degrees: i32) -> Position {
    // (sin, cos)
    let rotation = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
    let i: usize;

    if degrees > 0 {
        i = (degrees / 90) as usize;
    } else {
        i = (4 + (degrees / 90)) as usize;
    }

    Position {
        position: (
            rotation[i].1 * position.position.0 - rotation[i].0 * position.position.1,
            rotation[i].0 * position.position.0 + rotation[i].1 * position.position.1,
        ),
        direction: Direction::North,
    }
}

fn make_position(instruction: &Instruction, original_direction: Direction) -> Position {
    match instruction.direction {
        North => Position {
            position: (0, instruction.value as i32),
            direction: original_direction,
        },
        South => Position {
            position: (0, -(instruction.value as i32)),
            direction: original_direction,
        },
        East => Position {
            position: (instruction.value as i32, 0),
            direction: original_direction,
        },
        West => Position {
            position: (-(instruction.value as i32), 0),
            direction: original_direction,
        },
        Forward => make_position(
            &Instruction {
                direction: original_direction,
                value: instruction.value,
            },
            original_direction,
        ),
        _ => {
            let mut new_direction = original_direction;
            for _i in 0..instruction.value / 90 {
                new_direction = rotate_direction(new_direction, instruction.direction);
            }
            Position {
                position: (0, 0),
                direction: new_direction,
            }
        }
    }
}

fn rotate_direction(original_direction: Direction, instruction_direction: Direction) -> Direction {
    match original_direction {
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
