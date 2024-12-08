use crate::utils::line_integer_parser::parse_two_integers;
use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
pub struct Puzzle {
    left: Vec<i32>,
    right: Vec<i32>,
}

impl Puzzle {
    pub fn from_file(filename: &str) -> Puzzle {
        let (left, right): (Vec<_>, Vec<_>) = fs::read_to_string(filename)
            .expect("Error reading file")
            .lines()
            .map(|line| parse_two_integers(line).unwrap().1)
            .unzip();

        Self { left, right }
    }

    #[allow(dead_code)]
    pub fn new(left: Vec<i32>, right: Vec<i32>) -> Puzzle {
        Self { left, right }
    }

    pub fn calculate_distance(&self) -> i32 {
        let mut sorted_left = self.left.to_vec();
        let mut sorted_right = self.right.to_vec();

        sorted_left.sort();
        sorted_right.sort();

        sorted_left
            .iter()
            .zip(sorted_right.iter())
            .map(|(a, b)| (a - b).abs())
            .sum()
    }

    pub fn calculate_frequency(&self) -> i32 {
        let left_frequency_map = Self::build_frequency_map(&self.left);
        let right_frequency_map = Self::build_frequency_map(&self.right);

        left_frequency_map
            .iter()
            .map(|(number, left_freq)| {
                let right_freq = right_frequency_map.get(number).unwrap_or(&0);
                number * left_freq * right_freq
            })
            .sum()
    }

    fn build_frequency_map(numbers: &[i32]) -> HashMap<i32, i32> {
        let mut frequency = HashMap::new();

        for &number in numbers {
            *frequency.entry(number).or_insert(0) += 1;
        }

        frequency
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_distance() {
        // Arrange
        let left = vec![3, 4, 2, 1, 3, 3];
        let right = vec![4, 3, 5, 3, 9, 3];
        let puzzle = Puzzle::new(left, right);
        // Act
        let result = puzzle.calculate_distance();
        // Assert
        assert_eq!(result, 11);
    }

    #[test]
    fn test_calculate_frequency() {
        // Arrange
        let left = vec![3, 4, 2, 1, 3, 3];
        let right = vec![4, 3, 5, 3, 9, 3];
        let puzzle = Puzzle::new(left, right);
        // Act
        let result = puzzle.calculate_frequency();
        // Assert
        assert_eq!(result, 31);
    }

    #[test]
    fn test_build_frequency_map_duplicates() {
        // Arrange
        let numbers = vec![5, 5, 5, 2, 2, 3];
        // Act
        let result = Puzzle::build_frequency_map(&numbers);
        // Assert
        assert_eq!(result.get(&5), Some(&3));
        assert_eq!(result.get(&2), Some(&2));
        assert_eq!(result.get(&3), Some(&1));
        assert_eq!(result.get(&4), None);
    }
}
