use lazy_static::lazy_static;
use regex::Regex;
use std::{cell::Cell, fs, ops::Not};

const FILE_PATH: &str = "src/day_05/resources/input.txt";

struct Instruction {
    amount: u32,
    src: u32,
    dst: u32,
}

struct Stack {
    number: u32,
    crates: Cell<Vec<char>>,
}

impl Stack {
    fn add_crates(&self, new_crates: Vec<char>) {
        let mut mutable_crates = self.crates.take();

        mutable_crates.splice(0..0, new_crates);

        self.crates.set(mutable_crates)
    }

    fn remove_crates(&self, amount: usize, pick_multiple: bool) -> Vec<char> {
        let mut mutable_crates = self.crates.take();

        let mut removed_crates: Vec<char> = mutable_crates.drain(0..amount).collect();

        if pick_multiple.not() {
            removed_crates.reverse();
        }

        self.crates.set(mutable_crates);

        return removed_crates;
    }

    fn get_top_crate(&self) -> char {
        let local_crates = self.crates.take().clone();

        let top_item = local_crates.first().unwrap().clone();
        self.crates.set(local_crates);

        return top_item;
    }
}

fn get_instructions_data(raw_instructions: &str) -> Vec<Instruction> {
    let instructions: Vec<&str> = raw_instructions.split("\n").collect();

    lazy_static! {
        static ref AMOUNT_PATTERN: Regex = Regex::new(r"move (\d+)").unwrap();
        static ref SRC_PATTERN: Regex = Regex::new(r"from (\d+)").unwrap();
        static ref DST_PATTERN: Regex = Regex::new(r"to (\d+)").unwrap();
    }

    let instructions_data: Vec<Instruction> = instructions
        .iter()
        .map(|instruction| {
            let amount_match = AMOUNT_PATTERN.captures(&instruction).unwrap();
            let src_match = SRC_PATTERN.captures(&instruction).unwrap();
            let dst_match = DST_PATTERN.captures(&instruction).unwrap();

            return Instruction {
                amount: amount_match
                    .get(1)
                    .map_or(0, |m| m.as_str().parse::<u32>().unwrap()),
                src: src_match
                    .get(1)
                    .map_or(0, |m| m.as_str().parse::<u32>().unwrap()),
                dst: dst_match
                    .get(1)
                    .map_or(0, |m| m.as_str().parse::<u32>().unwrap()),
            };
        })
        .collect();

    instructions_data
}

fn get_stacks_data(raw_stacks: &str) -> Vec<Stack> {
    let mut stacks_rows: Vec<&str> = raw_stacks.split("\n").collect();
    let stacks_numbers: Vec<u32> = stacks_rows
        .pop()
        .unwrap()
        .split_whitespace()
        .map(|number| number.parse::<u32>().unwrap())
        .collect();
    stacks_rows.reverse();
    let max_crates_in_stack = stacks_rows.len();

    let stacks: Vec<Stack> = stacks_numbers
        .iter()
        .map(|stack_number| {
            let stack_value_index = 1 + ((stack_number - 1) * 4);

            let mut crates: Vec<char> = Vec::new();

            for n in 0..max_crates_in_stack {
                let stack_crate = stacks_rows[n]
                    .chars()
                    .nth(stack_value_index as usize)
                    .unwrap();

                if stack_crate.is_whitespace().not() {
                    crates.push(stack_crate);
                }
            }

            crates.reverse();

            Stack {
                number: *stack_number,
                crates: Cell::new(crates),
            }
        })
        .collect();

    stacks
}

fn get_input_data(content: &String) -> Option<(Vec<Instruction>, Vec<Stack>)> {
    if let [stacks, instructions] = content.split("\n\n").collect::<Vec<&str>>()[..] {
        let instructions = get_instructions_data(instructions);
        let stacks = get_stacks_data(stacks);

        return Some((instructions, stacks));
    }

    return None;
}

fn rearange_crates(
    instructions: &Vec<Instruction>,
    stacks: &Vec<Stack>,
    pick_multiple: bool,
) -> String {
    for instruction in instructions {
        let src_stack = stacks
            .iter()
            .find(|stack| stack.number == instruction.src)
            .unwrap();
        let dst_stack = stacks
            .iter()
            .find(|stack| stack.number == instruction.dst)
            .unwrap();

        let crates_to_move = src_stack.remove_crates(instruction.amount as usize, pick_multiple);
        dst_stack.add_crates(crates_to_move);
    }

    let result = stacks.iter().fold(String::new(), |acc, cur| {
        let mut acc_mut = acc.clone();
        acc_mut.push(cur.get_top_crate());

        return acc_mut;
    });

    return result;
}

pub fn execute() {
    let content = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");

    let input_data_1 = get_input_data(&content);
    let input_data_2 = get_input_data(&content);

    let (instructions, stacks) = input_data_1.unwrap();
    let result_1 = rearange_crates(&instructions, &stacks, false);

    let (instructions, stacks) = input_data_2.unwrap();
    let result_2 = rearange_crates(&instructions, &stacks, true);

    println!("Result 1 {}", result_1);
    println!("Result 2 {}", result_2);
}
