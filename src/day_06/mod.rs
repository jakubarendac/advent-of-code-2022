use std::{collections::HashSet, fs, str::Chars};

const FILE_PATH: &str = "src/day_06/resources/input.txt";
const PACKET_UNIQUE_CHARS_COUNT: usize = 4;
const MESSAGE_UNIQUE_CHARS_COUNT: usize = 14;

fn check_is_valid_sequence(
    already_read_items: &mut Vec<char>,
    sequence_start_offset: &mut i32,
    current_char: char,
    sequence_length: usize,
) -> bool {
    let mut sequence_to_check = already_read_items[(*sequence_start_offset as usize)
        ..(*sequence_start_offset as usize) + (sequence_length - 1)]
        .to_vec();
    sequence_to_check.push(current_char);
    let sequence_set: HashSet<char> = HashSet::from_iter(sequence_to_check);

    return sequence_set.len() == sequence_length;
}

fn check_next_buffer_item(
    already_read_items: &mut Vec<char>,
    sequence_start_offset: &mut i32,
    current_char: char,
    remaining_buffer: &mut Chars,
    sequence_length: usize,
) -> usize {
    if already_read_items.is_empty() || already_read_items.len() < sequence_length - 1 {
        already_read_items.push(current_char);
    } else {
        let is_valid = check_is_valid_sequence(
            already_read_items,
            sequence_start_offset,
            current_char,
            sequence_length,
        );

        if is_valid {
            return already_read_items.len() + 1;
        }

        already_read_items.push(current_char);
        *sequence_start_offset += 1;
    }

    check_next_buffer_item(
        already_read_items,
        sequence_start_offset,
        remaining_buffer.next().unwrap(),
        remaining_buffer,
        sequence_length,
    )
}

fn detect_sequence_of_size(length: usize, content: &String) -> usize {
    let mut buffer = content.chars();

    let mut already_read_items: Vec<char> = Vec::new();
    let mut sequence_start_offset = 0;

    let result = check_next_buffer_item(
        &mut already_read_items,
        &mut sequence_start_offset,
        buffer.next().unwrap(),
        &mut buffer,
        length,
    );

    return result;
}

pub fn execute() {
    let content = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");

    let result_1 = detect_sequence_of_size(PACKET_UNIQUE_CHARS_COUNT, &content);
    let result_2 = detect_sequence_of_size(MESSAGE_UNIQUE_CHARS_COUNT, &content);

    println!("Result 1 {}", result_1);
    println!("Result 2 {}", result_2);
}
