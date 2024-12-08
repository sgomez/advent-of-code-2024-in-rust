mod coords;
mod map_antinodes;
mod map_harmonics;

use crate::day08::map_antinodes::MapAntinodes;
use crate::day08::map_harmonics::MapHarmonics;

pub(crate) fn run() {
    let filename = "data/day08/input.txt";
    let mut map = MapAntinodes::from_file(filename);
    let result_part_1 = map.analyze();
    println!("Result of Day 08, Part 1 is {}", result_part_1);
    let mut map = MapHarmonics::from_file(filename);
    let result_part_2 = map.analyze();
    println!("Result of Day 08, Part 2 is {}", result_part_2);
}
