use crate::day10::trailhead::TrailHead;

mod trailhead;

pub(crate) fn run() {
    let filename = "data/day10/input.txt";

    let trail_head = TrailHead::from_file(&filename);
    let result_part_1 = trail_head.count_all_trails();
    println!("Result of Day 10, Part 1 is {}", result_part_1);

    let result_part_2 = trail_head.count_all_multiple_trails();
    println!("Result of Day 10, Part 1 is {}", result_part_2);
}
