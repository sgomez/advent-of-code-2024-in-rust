use bitflags::bitflags;
use std::fs;

pub(crate) fn run() {
    let filename = "data/day07/input.txt";
    let result_part_1 = resolve_part(filename, Operation::SUM | Operation::MULTIPLICATION);
    println!("Result of Day 06, Part 1 is {}", result_part_1);
    let result_part_2 = resolve_part(
        filename,
        Operation::SUM | Operation::MULTIPLICATION | Operation::CONCATENATION,
    );
    println!("Result of Day 06, Part 2 is {}", result_part_2);
}

fn resolve_part(filename: &str, operations: Operation) -> i64 {
    let contents = fs::read_to_string(filename).expect("Error reading file");

    contents
        .lines()
        .map(|line| {
            let equation = Equation::from_string(line, operations.clone());
            if equation.is_valid() {
                equation.result
            } else {
                0
            }
        })
        .sum()
}

bitflags! {
    #[derive(Debug, Clone)]
    pub struct Operation: u32 {
        const SUM = 0b0001;
        const MULTIPLICATION = 0b0010;
        const CONCATENATION = 0b0100;
    }
}

#[derive(Debug, Clone)]
pub struct Equation {
    result: i64,
    numbers: Vec<i64>,
    operations: Operation,
}

impl Equation {
    pub fn from_string(line: &str, operations: Operation) -> Equation {
        if let Some((left, right)) = line.split_once(": ") {
            let result = left.parse::<i64>().ok().unwrap();
            let numbers = right
                .split_whitespace()
                .filter_map(|x| x.parse::<i64>().ok())
                .collect::<Vec<i64>>();

            Self {
                result,
                numbers,
                operations,
            }
        } else {
            panic!("Invalid input");
        }
    }

    pub fn is_valid(&self) -> bool {
        return self.calculate_formula().contains(&self.result);
    }

    pub fn calculate_formula(&self) -> Vec<i64> {
        let mut stack: Vec<Vec<i64>> = vec![self.numbers.clone()];

        loop {
            if let Some(first_group) = stack.first() {
                if first_group.len() == 1 {
                    return stack
                        .iter()
                        .filter_map(|group| group.first().copied())
                        .collect();
                }
            } else {
                return vec![];
            }

            fn concatenate_numbers(a: i64, b: i64) -> i64 {
                let concatenated = format!("{}{}", a, b);
                concatenated.parse::<i64>().unwrap()
            }

            let mut next: Vec<Vec<i64>> = vec![];
            for group in stack {
                if self.operations.contains(Operation::SUM) {
                    let sum_group = Equation::create_operation(&group, |a, b| a + b);
                    if let Some(first) = sum_group.first() {
                        if *first <= self.result {
                            next.push(sum_group);
                        }
                    }
                }

                if self.operations.contains(Operation::MULTIPLICATION) {
                    let multi_group = Equation::create_operation(&group, |a, b| a * b);
                    if let Some(first) = multi_group.first() {
                        if *first <= self.result {
                            next.push(multi_group);
                        }
                    }
                }

                if self.operations.contains(Operation::CONCATENATION) {
                    let concat_group =
                        Equation::create_operation(&group, |a, b| concatenate_numbers(a, b));
                    if let Some(first) = concat_group.first() {
                        if *first <= self.result {
                            next.push(concat_group);
                        }
                    }
                }
            }
            stack = next;
        }
    }

    fn create_operation(from: &[i64], op: fn(i64, i64) -> i64) -> Vec<i64> {
        let (left, right) = from.split_at(2);

        let result = op(left[0], left[1]);
        let mut operations = vec![result];
        operations.extend_from_slice(right);
        operations
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn test_create_equation() {
        // Arrange
        let formula = "3267: 81 40 27";
        // Act
        let equation = Equation::from_string(formula, Operation::SUM | Operation::MULTIPLICATION);
        // Assert
        assert_eq!(equation.result, 3267);
        assert_eq!(equation.numbers, vec![81, 40, 27]);
    }

    #[test]
    fn test_calculate_formula() {
        // Arrange
        let equation =
            Equation::from_string("3267: 1 2 3", Operation::SUM | Operation::MULTIPLICATION);
        // Act
        let results = equation.calculate_formula();
        // Assert
        assert_eq!(results, vec![6, 9, 5, 6])
    }

    #[test_case("3267: 81 40 27", true)]
    #[test_case("21037: 9 7 18 13", false)]
    fn test_test_equation(formula: &str, expected: bool) {
        // Arrange
        let equation = Equation::from_string(formula, Operation::SUM | Operation::MULTIPLICATION);
        // Act
        let results = equation.is_valid();
        // Assert
        assert_eq!(results, expected)
    }

    #[test]
    fn test_resolve_part1() {
        // Arrange
        let filename = "data/day07/example.txt";
        // Act
        let results = resolve_part(filename, Operation::SUM | Operation::MULTIPLICATION);
        // Assert
        assert_eq!(results, 3749)
    }

    #[test]
    fn test_resolve_part2() {
        // Arrange
        let filename = "data/day07/example.txt";
        // Act
        let results = resolve_part(
            filename,
            Operation::SUM | Operation::MULTIPLICATION | Operation::CONCATENATION,
        );
        // Assert
        assert_eq!(results, 11387)
    }
}
