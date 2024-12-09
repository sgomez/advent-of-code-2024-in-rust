use crate::day09::disk::Disk;
use std::fs;

mod disk;

pub(crate) fn run() {
    let filename = "data/day09/input.txt";
    let content = fs::read_to_string(filename).unwrap();
    let first_line = content.lines().next().unwrap();

    let mut disk = Disk::from_string(first_line);
    disk.defragment();
    let result_part_1 = disk.checksum();
    println!("Result of Day 08, Part 1 is {}", result_part_1);

    let mut disk = Disk::from_string(first_line);
    disk.defragment_full();
    let result_part_2 = disk.checksum();
    println!("Result of Day 08, Part 2 is {}", result_part_2);
}
