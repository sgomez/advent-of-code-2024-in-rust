use crate::day04::read_matrix_from_file;

pub struct MatrixOne {
    matrix: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

impl MatrixOne {
    pub fn from_file(filename: &str) -> Self {
        let (matrix, rows, cols) = read_matrix_from_file(filename);
        Self { matrix, rows, cols }
    }

    pub(crate) fn count_xmas(&self) -> i32 {
        (0..self.rows)
            .map(|i| self.line(i))
            .chain((0..self.cols).map(|i| self.column(i)))
            .chain((0..(self.rows + self.cols - 1)).map(|i| self.diagonal_positive(i)))
            .chain((0..(self.rows + self.cols - 1)).map(|i| self.diagonal_negative(i)))
            .collect::<Vec<String>>()
            .iter()
            .map(|sequence| Self::count_xmas_words(sequence))
            .sum()
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

    fn count_xmas_words(line: &str) -> i32 {
        let patterns = ["XMAS", "SAMX"];
        patterns
            .iter()
            .map(|&pattern| line.match_indices(pattern).count() as i32)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn test_count_xmas() {
        // Arrange
        let line: Vec<&str> = "XMASAMXAMM".split("").collect();
        let joined = line.join("");
        // Act
        let count = MatrixOne::count_xmas_words(&joined);
        // Assert
        assert_eq!(count, 2);
    }

    #[test]
    fn test_create_matrix() {
        // Arrange
        let filename = "data/day04/example.txt";
        // Act
        let matrix = MatrixOne::from_file(&filename);
        // Assert
        assert_eq!(matrix.matrix[0][0], 'M');
        assert_eq!(matrix.matrix[2][0], 'A');
        assert_eq!(matrix.matrix[0][2], 'M');
    }

    #[test]
    fn test_get_line() {
        // Arrange
        let filename = "data/day04/example.txt";
        let matrix = MatrixOne::from_file(&filename);
        // Act
        let line = matrix.line(0);
        // Assert
        assert_eq!(line, "MMMSXXMASM");
    }

    #[test]
    fn test_get_column() {
        // Arrange
        let filename = "data/day04/example.txt";
        let matrix = MatrixOne::from_file(&filename);
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
        let filename = "data/day04/example.txt";
        let matrix = MatrixOne::from_file(&filename);
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
        let filename = "data/day04/example.txt";
        let matrix = MatrixOne::from_file(&filename);
        // Act
        let diagonal = matrix.diagonal_negative(index);
        // Assert
        assert_eq!(diagonal, expected);
    }

    #[test]
    fn test_matrix_count_xmas() {
        // Arrange
        let filename = "data/day04/example.txt";
        let matrix = MatrixOne::from_file(&filename);
        // Act
        let count = matrix.count_xmas();
        // Assert
        assert_eq!(count, 18);
    }
}
