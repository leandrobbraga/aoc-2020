extern crate strum;
#[macro_use]
extern crate strum_macros;

use std::collections::HashSet;
use std::fs;

#[derive(EnumString, Debug)]
enum Operations {
    #[allow(clippy::non_camel_case_types)]
    nop,
    acc,
    jmp,
}

struct Instruction {
    operation: Operations,
    argument: i32,
}

impl Instruction {
    fn new(unparsed_instruction: &str) -> Instruction {
        let unparsed_instruction: Vec<&str> = unparsed_instruction.split(" ").collect();
        Instruction {
            operation: unparsed_instruction[0].parse().unwrap(),
            argument: unparsed_instruction[1].parse().unwrap(),
        }
    }
}

fn main() {
    let input = fs::read_to_string("./examples/input/day-08.txt").unwrap();
    let mut instructions: Vec<Instruction> =
        input.lines().map(|line| Instruction::new(line)).collect();

    solve_part_1(&instructions);
    solve_part_2(&mut instructions);
}

fn solve_part_1(instructions: &Vec<Instruction>) {
    let (instruction_pointer, accumulator, _) = execute(instructions);

    println!("Part 1:");
    println!("Final instruction: {}", instruction_pointer);
    println!("Accumulator value: {}", accumulator);
}

fn execute(instructions: &Vec<Instruction>) -> (bool, i32, HashSet<usize>) {
    let mut instruction_pointer: usize = 0;
    let mut visited_instructions: HashSet<usize> = HashSet::new();
    let mut accumulator = 0;

    while instruction_pointer < instructions.len()
        && !visited_instructions.contains(&instruction_pointer)
    {
        visited_instructions.insert(instruction_pointer);

        let instruction: &Instruction = &instructions[instruction_pointer];
        match instruction.operation {
            Operations::nop => instruction_pointer += 1,
            Operations::acc => {
                instruction_pointer += 1;
                accumulator += instruction.argument;
            }
            Operations::jmp => {
                instruction_pointer = (instruction_pointer as i32 + instruction.argument) as usize
            }
        }
    }
    (
        instruction_pointer >= instructions.len(),
        accumulator,
        visited_instructions,
    )
}

fn solve_part_2(instructions: &mut Vec<Instruction>) {
    let (_, _, visited_instructions) = execute(instructions);

    for instruction_pointer in visited_instructions {
        instructions[instruction_pointer] = match flip_operation(&instructions[instruction_pointer])
        {
            None => continue,
            Some(instr) => instr,
        };

        let (terminated, acc, _) = execute(instructions);

        instructions[instruction_pointer] =
            flip_operation(&instructions[instruction_pointer]).unwrap();

        if terminated {
            println!("Part 2:");
            println!("Final instruction: {}", instruction_pointer);
            println!("Accumulator value: {}", acc);
            break;
        }
    }
}

fn flip_operation(instruction: &Instruction) -> Option<Instruction> {
    match instruction.operation {
        Operations::acc => None,
        Operations::nop => Some(Instruction {
            operation: Operations::jmp,
            argument: instruction.argument,
        }),
        Operations::jmp => Some(Instruction {
            operation: Operations::nop,
            argument: instruction.argument,
        }),
    }
}
