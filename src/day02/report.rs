use crate::utils::line_integer_parser::parse_line_of_integers;

#[derive(PartialEq, Debug)]
pub struct Report {
    data: Vec<i32>,
}

impl Report {
    pub fn new(data: Vec<i32>) -> Report {
        Self { data }
    }

    pub fn from_string(input: &str) -> Report {
        match parse_line_of_integers(input) {
            Ok((_, data)) => Self { data },
            Err(_) => panic!("Error parsing data"),
        }
    }

    pub fn is_safe(&self) -> bool {
        let mut prev = self.data[0];
        let is_ascending = self.data[1] > prev;

        for &current in &self.data[1..] {
            let difference = (current - prev).abs();

            if (is_ascending && current < prev)
                || (!is_ascending && current > prev)
                || difference < 1
                || difference > 3
            {
                return false;
            }

            prev = current;
        }

        true
    }

    pub fn is_safe_with_tolerance(&self) -> bool {
        if self.is_safe() {
            return true;
        }

        for i in 0..self.data.len() {
            let mut modified_data = self.data.clone();
            modified_data.remove(i);
            let modified_report = Report::new(modified_data);
            if modified_report.is_safe() {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![7, 6, 4, 2, 1], true)]
    #[test_case(vec![1, 2, 7, 8, 9], false)]
    #[test_case(vec![9, 7, 6, 2, 1], false)]
    #[test_case(vec![1, 3, 2, 4, 5], false)]
    #[test_case(vec![8, 6, 4, 4, 1], false)]
    #[test_case(vec![1, 3, 6, 7, 9], true)]
    fn test_report_is_safe(data: Vec<i32>, expected: bool) {
        // Arrange
        let report = Report::new(data);

        // Act
        let result = report.is_safe();

        // Assert
        assert_eq!(result, expected);
    }

    #[test_case(vec![7, 6, 4, 2, 1], true)]
    #[test_case(vec![1, 2, 7, 8, 9], false)]
    #[test_case(vec![9, 7, 6, 2, 1], false)]
    #[test_case(vec![1, 3, 2, 4, 5], true)]
    #[test_case(vec![8, 6, 4, 4, 1], true)]
    #[test_case(vec![1, 3, 6, 7, 9], true)]
    fn test_report_is_safe_with_tolerance(data: Vec<i32>, expected: bool) {
        // Arrange
        let report = Report::new(data);

        // Act
        let result = report.is_safe_with_tolerance();

        // Assert
        assert_eq!(result, expected);
    }
}
