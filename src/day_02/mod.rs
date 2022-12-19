#![allow(dead_code)]
#![allow(unused_variables)]
use std::fs;

const FILE_PATH: &str = "src/day_02/resources/input.txt";

enum Item {
    Rock,
    Paper,
    Scissors,
}

enum Result {
    Win,
    Lose,
    Draw,
}

fn get_item(raw_item: &str) -> Item {
    match raw_item {
        "A" | "X" => Item::Rock,
        "B" | "Y" => Item::Paper,
        "C" | "Z" => Item::Scissors,
        &_ => todo!(),
    }
}

fn check_result(you: &Item, opponent: &Item) -> Result {
    match you {
        Item::Rock => match opponent {
            Item::Scissors => Result::Win,
            Item::Paper => Result::Lose,
            Item::Rock => Result::Draw,
        },
        Item::Paper => match opponent {
            Item::Scissors => Result::Lose,
            Item::Paper => Result::Draw,
            Item::Rock => Result::Win,
        },
        Item::Scissors => match opponent {
            Item::Scissors => Result::Draw,
            Item::Paper => Result::Win,
            Item::Rock => Result::Lose,
        },
    }
}

fn get_result_value(result: &Result) -> i32 {
    match result {
        Result::Win => 6,
        Result::Draw => 3,
        Result::Lose => 0,
    }
}

fn get_item_value(item: &Item) -> i32 {
    match item {
        Item::Rock => 1,
        Item::Paper => 2,
        Item::Scissors => 3,
    }
}

fn get_wanted_result(raw_wanted_result: &str) -> Result {
    match raw_wanted_result {
        "X" => Result::Lose,
        "Y" => Result::Draw,
        "Z" => Result::Win,
        &_ => todo!(),
    }
}

fn get_your_item_by_result_and_oponnent(opponent: &Item, wanted_result: &Result) -> Item {
    match wanted_result {
        Result::Win => match opponent {
            Item::Scissors => Item::Rock,
            Item::Paper => Item::Scissors,
            Item::Rock => Item::Paper,
        },
        Result::Draw => match opponent {
            Item::Scissors => Item::Scissors,
            Item::Paper => Item::Paper,
            Item::Rock => Item::Rock,
        },
        Result::Lose => match opponent {
            Item::Scissors => Item::Paper,
            Item::Paper => Item::Rock,
            Item::Rock => Item::Scissors,
        },
    }
}

fn get_round_score_1(round: &Vec<&str>) -> i32 {
    if let [oponnent, you] = &round[..] {
        let your_item = get_item(you);
        let opponent_item = get_item(oponnent);

        let your_item_value = get_item_value(&your_item);
        let result_value = get_result_value(&check_result(&your_item, &opponent_item));

        return your_item_value + result_value;
    }

    return 0;
}

fn get_round_score_2(round: &Vec<&str>) -> i32 {
    if let [oponnent, you] = &round[..] {
        let oponent_item = get_item(oponnent);
        let wanted_result = get_wanted_result(you);
        let your_item = get_your_item_by_result_and_oponnent(&oponent_item, &wanted_result);

        return get_result_value(&wanted_result) + get_item_value(&your_item);
    }
    return 0;
}

pub fn execute() {
    let content = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");

    let rounds: Vec<Vec<&str>> = content
        .split("\n")
        .map(|round| round.split(" ").collect())
        .collect();

    let result_1: i32 = rounds.iter().map(get_round_score_1).sum();
    let result_2: i32 = rounds.iter().map(get_round_score_2).sum();

    println!("Result 1 {}", result_1);
    println!("Result 2 {}", result_2);
}
