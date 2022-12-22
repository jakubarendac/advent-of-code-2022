#![allow(dead_code)]
#![allow(unused_variables)]
use std::collections::HashSet;
use std::fs;

const FILE_PATH: &str = "src/day_03/resources/input.txt";

fn get_letter_value(letter: char) -> u32 {
    if letter.is_ascii_uppercase() {
        return letter as u32 - 38;
    }
    if letter.is_ascii_lowercase() {
        return letter as u32 - 96;
    }

    return 0;
}

fn get_rucksack_common_items_price(comparments: &Vec<String>) -> u32 {
    let common_items = comparments.iter().map(|s| s.to_string()).fold(
        HashSet::new(),
        |acc: HashSet<u32>, cur: String| {
            let nxt_values: HashSet<u32> = cur.chars().map(get_letter_value).collect();

            let intersected = acc
                .intersection(&nxt_values)
                .map(|i| *i)
                .collect::<HashSet<_>>();

            if intersected.is_empty() {
                return nxt_values;
            }

            return intersected;
        },
    );

    return common_items.iter().sum();
}

pub fn execute() {
    let content = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");

    let rucksacks: Vec<Vec<char>> = content
        .split("\n")
        .map(|rucksack| rucksack.chars().collect())
        .collect();

    let rucksacks_compartments = rucksacks
        .iter()
        .map(|rucksack| {
            rucksack
                .chunks(rucksack.len() / 2)
                .map(|compartment| compartment.iter().collect::<String>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let elfs_groups = rucksacks
        .chunks(3)
        .map(|rucksack| {
            rucksack
                .iter()
                .map(|item| item.iter().collect::<String>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let result_1: u32 = rucksacks_compartments
        .iter()
        .map(get_rucksack_common_items_price)
        .sum();

    let result_2: u32 = elfs_groups
        .iter()
        .map(get_rucksack_common_items_price)
        .sum();

    println!("Result 1 {}", result_1);
    println!("Result 2 {}", result_2);
}
