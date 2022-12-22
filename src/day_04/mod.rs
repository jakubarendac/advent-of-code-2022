use std::fs;
use std::ops::Range;

const FILE_PATH: &str = "src/day_04/resources/input.txt";

fn create_pair_item_range(item: &str) -> Range<i32> {
    let indices: Vec<i32> = item
        .split("-")
        .map(|index| index.parse().unwrap())
        .collect();

    if let [start_index, end_index] = indices[..] {
        return Range {
            start: start_index,
            end: end_index + 1,
        };
    }

    return Range { start: 0, end: 0 };
}

fn check_pair_whole_subrange_reducer(acc: i32, cur: &Vec<Range<i32>>) -> i32 {
    if let [first_range, second_range] = &cur[..] {
        if &first_range.len() > &second_range.len() {
            let are_boundaries_in_range = second_range.clone().min() >= first_range.clone().min()
                && second_range.clone().max() <= first_range.clone().max();

            if are_boundaries_in_range {
                return acc + 1;
            }

            return acc;
        }

        let are_boundaries_in_range = first_range.clone().min() >= second_range.clone().min()
            && first_range.clone().max() <= second_range.clone().max();

        if are_boundaries_in_range {
            return acc + 1;
        }

        return acc;
    }

    return acc;
}

fn check_pair_partial_subrange_reducer(acc: i32, cur: &Vec<Range<i32>>) -> i32 {
    if let [first_range, second_range] = &cur[..] {
        let is_any_element_contained = first_range
            .clone()
            .any(|item| second_range.clone().contains(&item));

        if is_any_element_contained {
            return acc + 1;
        }

        return acc;
    }

    return acc;
}

pub fn execute() {
    let content = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");

    let pairs_ranges: Vec<Vec<Range<i32>>> = content
        .split("\n")
        .map(|pair| pair.split(",").map(create_pair_item_range).collect())
        .collect();

    let result_1 = pairs_ranges
        .iter()
        .fold(0, check_pair_whole_subrange_reducer);
    let result_2 = pairs_ranges
        .iter()
        .fold(0, check_pair_partial_subrange_reducer);

    println!("Result 1 {}", { result_1 });
    println!("Result 2 {}", { result_2 });
}
