use std::fs;

const FILE_PATH: &str = "day_01/resources/input.txt";

pub fn execute() {
    // raw file text content
    let content = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");

    // vector of strings each representing one elf
    let elfs: Vec<&str> = content.split("\n\n").collect();

    // vector of vectors of string each representing one food
    let elfs_foods_string: Vec<Vec<&str>> =
        elfs.iter().map(|elf| elf.split("\n").collect()).collect();

    // vector of vectors of numbers each representing one food that could be summed later
    let elfs_foods_number: Vec<Vec<i32>> = elfs_foods_string
        .iter()
        .map(|elf| {
            elf.iter()
                .map(|food| {
                    food.strip_suffix("\r\n")
                        .or(food.strip_suffix("\n"))
                        .unwrap_or(food)
                        .parse::<i32>()
                        .unwrap_or(0)
                })
                .collect()
        })
        .collect();

    let mut elfs_foods_sums: Vec<i32> = elfs_foods_number
        .iter()
        .map(|item| item.iter().sum())
        .collect();

    elfs_foods_sums.sort();
    elfs_foods_sums.reverse();

    // max sum of foods for elf
    let result_1 = elfs_foods_sums.iter().max().unwrap();

    // sum of best 3 elfs foods
    let result_2: i32 = elfs_foods_sums.iter().take(3).sum();

    println!("Result 1 {}", result_1);
    println!("Result 2 {}", result_2);
}
