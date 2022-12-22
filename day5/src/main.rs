use std::fs;
use std::cmp;

fn rotate_matrix_90_counter_clockwise<T>(matrix: &mut Vec<Vec<T>>)
where T: Clone {
    let rows_len = matrix.len();
    let cols_len = matrix[0].len();

    let width = cmp::max(rows_len, cols_len);
    let mut padded = vec![vec![None; width]; width];

    for i in 0..rows_len {
        for j in 0..cols_len {
            padded[i][j] = Some(matrix[i][j].clone());
        }
    }

    for i in 0..(width/2 - 1) {
        for j in i..(width - i) {
            let temp = padded[i][j];
            padded[i][j] = padded[j][width - i - 1];
            padded[j][width - i - 1] = padded[width - i - 1][width - j - 1];
            padded[width - i - 1][width - j - 1] = padded[width - j - 1][i];
            padded[width - j - 1][i] = temp;
        }
    }
}

fn main() {
    let contents = fs::read_to_string("./input").unwrap();
    let lines: Vec<&str> = contents.split('\n').collect();

    // let mut table_reversed: Vec<&str> = Vec::new();
    let mut table_reversed: Vec<Vec<char>> = Vec::new();
    let mut table: Vec<&str> = Vec::new();
    let mut instructions: Vec<&str> = Vec::new();
    let mut table_finished: bool = false;

    for line in lines {
        if line.is_empty() {
            table_finished = true;
            continue;
        }
        if ! table_finished {
            table_reversed.push(line.chars().collect());
        } else {
            instructions.push(line);
        }
    }

    // let filtered: Vec<&str> = table_reversed.last().unwrap().split(' ').filter(|x| x.len() > 0).collect();
    // let columns: Vec<&str> = table_reversed.last().unwrap().split(' ').collect();
    // let columns2: Vec<&str> = table_reversed[table_reversed.len() - 2].split(' ').collect();

    rotate_matrix_90_counter_clockwise(&mut table_reversed);
    // println!("table_reversed: {table_reversed:?}");
    // println!("instructions: {instructions:?}");
    // println!("filtered: {filtered:?}");
    // println!("columns: {columns:?}");
    // println!("columns2: {columns2:?}");
}
