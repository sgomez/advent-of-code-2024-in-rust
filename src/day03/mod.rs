use once_cell::sync::Lazy;
use regex::Regex;
use std::fs;

static MUL_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"mul\((\d+),(\d+)\)").unwrap());
static EXTENDED_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap());

pub(crate) fn run() {
    let filename = "data/day03/input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let result_part_1 = parse_memory(&contents);
    println!("Result of Day 03, Part 1 is {}", result_part_1);
    let result_part_2 = parse_memory_extended(&contents);
    println!("Result of Day 03, Part 2 is {}", result_part_2);
}

fn parse_memory(input: &str) -> i32 {
    MUL_RE
        .captures_iter(input)
        .filter_map(|caps| {
            let left = caps.get(1)?.as_str().parse::<i32>().ok();
            let right = caps.get(2)?.as_str().parse::<i32>().ok();
            Some(left? * right?)
        })
        .sum()
}

fn parse_memory_extended(input: &str) -> i32 {
    let mut enabled = true;

    EXTENDED_RE
        .find_iter(input)
        .filter_map(|mat| match mat.as_str() {
            "do()" => {
                enabled = true;
                None
            }
            "don't()" => {
                enabled = false;
                None
            }
            mul if enabled => Some(parse_memory(mul)),
            _ => None,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_memory() {
        // Arrange
        let memory = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        // Act
        let result = parse_memory(memory);

        assert_eq!(result, 161);
    }

    #[test]
    fn test_parse_memory_extended() {
        // Arrange
        let memory = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        // Act
        let result = parse_memory_extended(memory);

        assert_eq!(result, 48);
    }
}
