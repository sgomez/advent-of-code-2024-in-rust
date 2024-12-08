mod matrix_one;
mod matrix_two;

use crate::day04::matrix_one::MatrixOne;
use crate::day04::matrix_two::MatrixTwo;
use std::fs;

pub(crate) fn run() {
    let filename = "data/day04/input.txt";
    let matrix = MatrixOne::from_file(filename);
    println!("Result of Day 04, Part 1 is {}", matrix.count_xmas());
    let matrix = MatrixTwo::from_file(filename);
    println!("Result of Day 04, Part 2 is {}", matrix.count_xmas_second());
}

fn read_matrix_from_file(filename: &str) -> (Vec<Vec<char>>, usize, usize) {
    let matrix = fs::read_to_string(filename)
        .expect("Error reading file")
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let rows = matrix.len();
    let cols = matrix.first().unwrap_or(&vec![]).len();

    (matrix, rows, cols)
}
