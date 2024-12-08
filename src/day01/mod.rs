mod puzzle;

use crate::day01::puzzle::Puzzle;

pub(crate) fn run() {
    let filename = "data/day01/input.txt";
    let puzzle = Puzzle::from_file(filename);

    let result_part_1 = puzzle.calculate_distance();
    println!("Result of Day 01, Part 1 is {}", result_part_1);

    let result_part_2 = puzzle.calculate_frequency();
    println!("Result of Day 01, Part 2 is {}", result_part_2);
}
