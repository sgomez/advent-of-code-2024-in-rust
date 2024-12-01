use crate::utils::read_lines;
use std::collections::HashMap;

pub fn calculate_distance(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    let mut sorted_left = left.to_vec();
    let mut sorted_right = right.to_vec();

    sorted_left.sort();
    sorted_right.sort();

    sorted_left
        .iter()
        .zip(sorted_right.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

pub fn calculate_frequency(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    let left_frequency_map = build_frequency_map(left);
    let right_frequency_map = build_frequency_map(right);

    left_frequency_map
        .iter()
        .map(|(number, left_freq)| {
            let right_freq = right_frequency_map.get(number).unwrap_or(&0);
            number * left_freq * right_freq
        })
        .sum()
}

pub fn build_frequency_map(numbers: &[i32]) -> HashMap<i32, i32> {
    let mut frequency = HashMap::new();

    for &number in numbers {
        *frequency.entry(number).or_insert(0) += 1;
    }

    frequency
}

fn parse_file_data(filename: &str) -> Result<(Vec<i32>, Vec<i32>), String> {
    let mut left = Vec::new();
    let mut right = Vec::new();

    let lines = read_lines(filename).map_err(|e| format!("Error reading file: {}", e))?;

    for line in lines {
        let line_content = line.map_err(|e| format!("Error reading line: {}", e))?;

        let numbers: Vec<i32> = line_content
            .split_whitespace()
            .map(|num| num.parse::<i32>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| format!("Failed to parse line: {}", line_content))?;

        if numbers.len() == 2 {
            left.push(numbers[0]);
            right.push(numbers[1]);
        } else {
            return Err(format!("Invalid line format: {}", line_content));
        }
    }

    Ok((left, right))
}

pub(crate) fn run() {
    let filename = "data/day01/input.txt";

    let (left, right) = parse_file_data(filename).expect("Error parsing file data");

    let result_part_1 = calculate_distance(&left, &right);
    println!("Result of problem 1 is: {}", result_part_1);

    let result_part_2 = calculate_frequency(&left, &right);
    println!("Result of problem 2 is: {}", result_part_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_distance() {
        // Arrange
        let left = vec![3, 4, 2, 1, 3, 3];
        let right = vec![4, 3, 5, 3, 9, 3];

        // Act
        let result = calculate_distance(&left, &right);

        // Assert
        assert_eq!(result, 11);
    }

    #[test]
    fn test_calculate_frequency() {
        // Arrange
        let left = vec![3, 4, 2, 1, 3, 3];
        let right = vec![4, 3, 5, 3, 9, 3];

        // Act
        let result = calculate_frequency(&left, &right);

        // Assert
        assert_eq!(result, 31);
    }

    #[test]
    fn test_build_frequency_map_duplicates() {
        // Arrange
        let numbers = vec![5, 5, 5, 2, 2, 3];

        // Act
        let result = build_frequency_map(&numbers);

        // Assert
        assert_eq!(result.get(&5), Some(&3));
        assert_eq!(result.get(&2), Some(&2));
        assert_eq!(result.get(&3), Some(&1));
        assert_eq!(result.get(&4), None);
    }
}
