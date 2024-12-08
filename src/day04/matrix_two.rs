use crate::day04::read_matrix_from_file;

pub struct MatrixTwo {
    matrix: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

impl MatrixTwo {
    pub fn from_file(filename: &str) -> Self {
        let (matrix, rows, cols) = read_matrix_from_file(filename);
        Self { matrix, rows, cols }
    }

    pub fn count_xmas_second(&self) -> i32 {
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

    fn has_xmas(&self, x: usize, y: usize) -> bool {
        if self.get(x, y) != Some(&'A') {
            return false;
        }

        let diagonal1 = (self.get(x - 1, y - 1), self.get(x + 1, y + 1));
        let diagonal2 = (self.get(x - 1, y + 1), self.get(x + 1, y - 1));

        let is_xmas_diagonal = |diagonal: (Option<&char>, Option<&char>)| {
            matches!(diagonal, (Some('M'), Some('S')) | (Some('S'), Some('M')))
        };

        is_xmas_diagonal(diagonal1) && is_xmas_diagonal(diagonal2)
    }

    fn get(&self, row: usize, col: usize) -> Option<&char> {
        self.matrix.get(row).and_then(|row| row.get(col))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_found_mas() {
        // Arrange
        let filename = "data/day04/example.txt";
        let matrix = MatrixTwo::from_file(&filename);
        // Act
        let found = matrix.has_xmas(1, 2);
        // Assert
        assert!(found);
    }

    #[test]
    fn test_matrix_count_xmas_second() {
        // Arrange
        let filename = "data/day04/example.txt";
        let matrix = MatrixTwo::from_file(&filename);
        // Act
        let count = matrix.count_xmas_second();
        // Assert
        assert_eq!(count, 9);
    }
}
