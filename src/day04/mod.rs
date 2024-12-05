use crate::utils::read_lines;

pub(crate) fn run() {
    let filename = "data/day04/input.txt";
    let matrix = parse_file_data(filename);

    println!("Result of Day 04, Part 1 is {}", matrix.count_xmas());
    println!("Result of Day 04, Part 2 is {}", matrix.count_xmas_second());
}

struct Matrix {
    matrix: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

fn parse_file_data(filename: &str) -> Matrix {
    let lines = read_lines(filename)
        .expect("Failed to read lines from file")
        .filter_map(|line| line.ok())
        .collect::<Vec<String>>();

    let lines_ref: Vec<&str> = lines.iter().map(AsRef::as_ref).collect();

    Matrix::new(&lines_ref)
}

impl Matrix {
    fn new(lines: &Vec<&str>) -> Self {
        let matrix = lines
            .iter()
            .map(|line| line.chars().collect())
            .collect::<Vec<Vec<char>>>();

        let rows = matrix.len();
        let cols = matrix.first().map_or(0, |r| r.len());

        Matrix { matrix, rows, cols }
    }

    fn get(&self, row: usize, col: usize) -> Option<&char> {
        match (row, col) {
            (x, _) if x >= self.rows => None,
            (_, x) if x >= self.cols => None,
            _ => self.matrix.get(row).and_then(|row| row.get(col)),
        }
    }

    fn has_xmas(&self, x: usize, y: usize) -> bool {
        if let Some('A') = self.get(x, y) {
            if !matches!(
                (self.get(x - 1, y - 1), self.get(x + 1, y + 1)),
                (Some('M'), Some('S')) | (Some('S'), Some('M'))
            ) {
                return false;
            }

            if !matches!(
                (self.get(x - 1, y + 1), self.get(x + 1, y - 1)),
                (Some('S'), Some('M')) | (Some('M'), Some('S'))
            ) {
                return false;
            }

            return true;
        }

        false
    }

    fn count_xmas(&self) -> i32 {
        let line_sum: i32 = (0..self.rows).map(|i| count_xmas(&self.line(i))).sum();
        let column_sum: i32 = (0..self.cols).map(|i| count_xmas(&self.column(i))).sum();

        let diagonals = self.rows + self.cols - 1;

        let diagonal_positive: i32 = (0..diagonals)
            .map(|i| count_xmas(&self.diagonal_positive(i)))
            .sum();

        let diagonal_negative: i32 = (0..diagonals)
            .map(|i| count_xmas(&self.diagonal_negative(i)))
            .sum();

        line_sum + column_sum + diagonal_positive + diagonal_negative
    }

    fn count_xmas_second(&self) -> i32 {
        let mut count = 0;

        for row in 1..self.rows {
            for col in 1..self.cols {
                if self.has_xmas(row, col) {
                    count += 1;
                }
            }
        }

        count
    }

    fn line(&self, index: usize) -> String {
        self.matrix.get(index).unwrap().iter().collect()
    }

    fn column(&self, index: usize) -> String {
        self.matrix
            .iter()
            .filter_map(|row| row.get(index))
            .collect()
    }

    fn diagonal_positive(&self, index: usize) -> String {
        let (mut row, mut col): (usize, usize) = if index > self.rows {
            (self.rows, index - self.rows)
        } else {
            (index, 0)
        };

        let mut diagonal = String::new();

        while col < self.cols {
            if let Some(value) = self.matrix.get(row).and_then(|row| row.get(col)) {
                diagonal.push(*value);
            }

            if row == 0 {
                break;
            }

            row -= 1;
            col += 1;
        }

        diagonal
    }

    fn diagonal_negative(&self, index: usize) -> String {
        let (mut row, mut col): (usize, usize) = if index < self.rows {
            (self.rows - index - 1, 0)
        } else {
            (0, index - self.rows + 1)
        };

        let mut diagonal = String::new();

        while row < self.rows && col < self.cols {
            if let Some(value) = self.matrix.get(row).and_then(|row| row.get(col)) {
                diagonal.push(*value);
            }

            row += 1;
            col += 1;
        }

        diagonal
    }
}

fn count_xmas(line: &String) -> i32 {
    let patterns = ["XMAS", "SAMX"];
    let mut count = 0;

    for pattern in patterns {
        let mut start = 0;
        while let Some(pos) = line[start..].find(pattern) {
            count += 1;
            start += pos + 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    fn example() -> Vec<&'static str> {
        vec![
            "MMMSXXMASM",
            "MSAMXMSMSA",
            "AMXSXMAAMM",
            "MSAMASMSMX",
            "XMASAMXAMM",
            "XXAMMXXAMA",
            "SMSMSASXSS",
            "SAXAMASAAA",
            "MAMMMXMMMM",
            "MXMXAXMASX",
        ]
    }

    #[test]
    fn test_count_xmas() {
        // Arrange
        let line: Vec<&str> = "XMASAMXAMM".split("").collect();
        let joined = line.join("");

        // Act
        let count = count_xmas(&joined);
        // Assert
        assert_eq!(count, 2);
    }

    #[test]
    fn test_create_matrix() {
        // Arrange
        let lines = example();
        // Act
        let matrix = Matrix::new(&lines);
        // Assert
        assert_eq!(matrix.matrix[0][0], 'M');
        assert_eq!(matrix.matrix[2][0], 'A');
        assert_eq!(matrix.matrix[0][2], 'M');
    }

    #[test]
    fn test_get_line() {
        // Arrange
        let lines = example();
        let matrix = Matrix::new(&lines);
        // Act
        let line = matrix.line(0);
        // Assert
        assert_eq!(line, "MMMSXXMASM");
    }

    #[test]
    fn test_get_column() {
        // Arrange
        let lines = example();
        let matrix = Matrix::new(&lines);
        // Act
        let line = matrix.column(0);
        // Assert
        assert_eq!(line, "MMAMXXSSMM");
    }

    #[test_case(0, "M")]
    #[test_case(1, "MM")]
    #[test_case(17, "SM")]
    #[test_case(18, "X")]
    fn test_diagonal_positive(index: usize, expected: &str) {
        // Arrange
        let lines = example();
        let matrix = Matrix::new(&lines);
        // Act
        let diagonal = matrix.diagonal_positive(index);
        // Assert
        assert_eq!(diagonal, expected);
    }

    #[test_case(0, "M")]
    #[test_case(1, "MX")]
    #[test_case(17, "SA")]
    #[test_case(18, "M")]
    fn test_diagonal_negative(index: usize, expected: &str) {
        // Arrange
        let lines = example();
        let matrix = Matrix::new(&lines);
        // Act
        let diagonal = matrix.diagonal_negative(index);
        // Assert
        assert_eq!(diagonal, expected);
    }

    #[test]
    fn test_matrix_count_xmas() {
        // Arrange
        let lines = example();
        let matrix = Matrix::new(&lines);

        // Act
        let count = matrix.count_xmas();

        // Assert
        assert_eq!(count, 18);
    }

    #[test]
    fn test_matrix_found_mas() {
        // Arrange
        let lines = example();
        let matrix = Matrix::new(&lines);

        // Act
        let found = matrix.has_xmas(1, 2);

        // Assert
        assert!(found);
    }

    #[test]
    fn test_matrix_count_xmas_second() {
        // Arrange
        let lines = example();
        let matrix = Matrix::new(&lines);

        // Act
        let count = matrix.count_xmas_second();

        // Assert
        assert_eq!(count, 9);
    }
}
