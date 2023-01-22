use std::fs;

const FILE_PATH: &str = "src/day_08/resources/input.txt";

fn get_tree_views(
    trees_matrix: &Vec<Vec<u32>>,
    row_index: usize,
    column_index: usize,
) -> (Vec<u32>, Vec<u32>, Vec<u32>, Vec<u32>) {
    let left_view_trees = trees_matrix[row_index][..column_index].to_owned();
    let right_view_trees = trees_matrix[row_index][column_index + 1..].to_owned();
    let top_view_trees = trees_matrix[..row_index]
        .iter()
        .map(|item| *item.get(column_index).unwrap())
        .collect::<Vec<u32>>();
    let bottom_view_trees = trees_matrix[row_index + 1..]
        .iter()
        .map(|item| *item.get(column_index).unwrap())
        .collect::<Vec<u32>>()
        .clone()
        .to_owned();

    let views = (
        left_view_trees,
        right_view_trees,
        top_view_trees,
        bottom_view_trees,
    );

    return views;
}

fn check_is_tree_visible(
    trees_matrix: &Vec<Vec<u32>>,
    row_index: usize,
    column_index: usize,
) -> bool {
    let tree_to_check = trees_matrix
        .get(row_index)
        .unwrap()
        .get(column_index)
        .unwrap();
    let (left_view_trees, right_view_trees, top_view_trees, bottom_view_trees) =
        get_tree_views(trees_matrix, row_index, column_index);

    let is_visible_from_left = &left_view_trees.iter().all(|tree| tree < tree_to_check);
    let is_visible_from_right = &right_view_trees.iter().all(|tree| tree < tree_to_check);
    let is_visible_from_top = &top_view_trees.iter().all(|tree| tree < &tree_to_check);
    let is_visible_from_bottom = &bottom_view_trees.iter().all(|tree| tree < &tree_to_check);

    let is_visible = *is_visible_from_left
        || *is_visible_from_right
        || *is_visible_from_top
        || *is_visible_from_bottom;

    return is_visible;
}

fn get_direction_visible_trees_count(tree_to_check: &u32, trees: &Vec<u32>) -> usize {
    let visible_trees_count = &trees
        .iter()
        .take_while(|tree| return *tree < tree_to_check)
        .collect::<Vec<_>>()
        .len();

    if *visible_trees_count < trees.len() {
        return visible_trees_count + 1;
    }

    return *visible_trees_count;
}

fn get_tree_scenic_score(
    trees_matrix: &Vec<Vec<u32>>,
    row_index: usize,
    column_index: usize,
) -> usize {
    let tree_to_check = trees_matrix
        .get(row_index)
        .unwrap()
        .get(column_index)
        .unwrap();
    let (left_view_trees, right_view_trees, top_view_trees, bottom_view_trees) =
        get_tree_views(trees_matrix, row_index, column_index);

    let mut left_view_trees_copy = left_view_trees.clone();
    left_view_trees_copy.reverse();

    let mut top_view_trees_copy = top_view_trees.clone();
    top_view_trees_copy.reverse();

    let left_visible_trees_count =
        get_direction_visible_trees_count(tree_to_check, &left_view_trees_copy);
    let right_visible_trees_count =
        get_direction_visible_trees_count(tree_to_check, &right_view_trees);
    let top_visible_trees_count =
        get_direction_visible_trees_count(tree_to_check, &top_view_trees_copy);
    let bottom_visible_trees_count =
        get_direction_visible_trees_count(tree_to_check, &bottom_view_trees);

    return left_visible_trees_count
        * right_visible_trees_count
        * top_visible_trees_count
        * bottom_visible_trees_count;
}

fn get_visible_trees_count(trees_matrix: &Vec<Vec<u32>>) -> usize {
    let rows_count = trees_matrix.len();
    let columns_count = trees_matrix.first().unwrap().len();

    let border_trees = 2 * rows_count + 2 * columns_count - 4;
    let mut inner_visible_trees = 0;

    for i in 1..(rows_count - 1) {
        for j in 1..(columns_count - 1) {
            let is_tree_visible = check_is_tree_visible(trees_matrix, i, j);

            if is_tree_visible {
                inner_visible_trees += 1;
            }
        }
    }

    return border_trees + inner_visible_trees;
}

fn get_highest_tree_scenic_score(trees_matrix: &Vec<Vec<u32>>) -> usize {
    let rows_count = trees_matrix.len();
    let columns_count = trees_matrix.first().unwrap().len();

    let mut highest_tree_scenic_score = 0;

    for i in 1..(rows_count - 1) {
        for j in 1..(columns_count - 1) {
            let tree_scenic_score = get_tree_scenic_score(trees_matrix, i, j);

            if tree_scenic_score > highest_tree_scenic_score {
                highest_tree_scenic_score = tree_scenic_score;
            }
        }
    }

    return highest_tree_scenic_score;
}

pub fn execute() {
    let content = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");

    let trees_matrix: Vec<Vec<u32>> = content
        .split("\n")
        .map(|row| row.chars().map(|item| item.to_digit(10).unwrap()).collect())
        .collect();

    let result_1 = get_visible_trees_count(&trees_matrix);
    let result_2 = get_highest_tree_scenic_score(&trees_matrix);

    println!("Result 1 {}", result_1);
    println!("Result 2 {}", result_2);
}
