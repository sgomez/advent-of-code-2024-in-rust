mod report;

use crate::day02::report::Report;
use crate::utils::read_lines;

pub(crate) fn run() {
    let filename = "data/day02/input.txt";
    let reports = parse_file_data(filename);

    let result_part_1 = calculate_valid_reports(&reports);
    println!("Result of Day 02, Part 1 is {}", result_part_1);
    let result_part_2 = calculate_valid_safe_reports(&reports);
    println!("Result of Day 02, Part 2 is {}", result_part_2);
}

fn calculate_valid_reports(reports: &Vec<Report>) -> usize {
    let safe_count = reports.iter().filter(|report| report.is_safe()).count();

    safe_count
}

fn calculate_valid_safe_reports(reports: &Vec<Report>) -> usize {
    let safe_count = reports
        .iter()
        .filter(|report| report.is_safe_with_tolerance())
        .count();

    safe_count
}

fn parse_file_data(filename: &str) -> Vec<Report> {
    read_lines(filename)
        .expect("File not found")
        .filter_map(|line| {
            line.ok()
                .map(|content| {
                    content
                        .split_whitespace()
                        .filter_map(|s| s.parse::<i32>().ok())
                        .collect::<Vec<i32>>()
                })
                .map(Report::new)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combined_reports() {
        // Arrange
        let reports = vec![
            Report::new(vec![7, 6, 4, 2, 1]), // Safe
            Report::new(vec![1, 2, 7, 8, 9]), // Unsafe
            Report::new(vec![9, 7, 6, 2, 1]), // Unsafe
            Report::new(vec![1, 3, 2, 4, 5]), // Unsafe
            Report::new(vec![8, 6, 4, 4, 1]), // Unsafe
            Report::new(vec![1, 3, 6, 7, 9]), // Safe
        ];

        // Act
        let result = calculate_valid_reports(&reports);

        // Assert
        assert_eq!(result, 2);
    }
}
