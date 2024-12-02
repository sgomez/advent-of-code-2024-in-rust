#[derive(PartialEq, Debug)]
pub struct Report {
    data: Vec<i32>,
}

impl Report {
    pub fn new(data: Vec<i32>) -> Report {
        Self { data }
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

    pub fn is_safe(&self) -> bool {
        let mut prev = self.data[0];
        let next = self.data[1];
        let is_ascending = next > prev;

        for &current in &self.data[1..] {
            let difference = (current - prev).abs();

            match is_ascending {
                true if current < prev => {
                    return false;
                }
                false if current > prev => {
                    return false;
                }
                _ if difference < 1 || difference > 3 => {
                    return false;
                }
                _ => {
                    prev = current;
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![7, 6, 4, 2, 1], true; "levels decreasing by 1 or 2")]
    #[test_case(vec![1, 2, 7, 8, 9], false; "increase of 5 between 2 and 7")]
    #[test_case(vec![9, 7, 6, 2, 1], false; "decrease of 4 between 6 and 2")]
    #[test_case(vec![1, 3, 2, 4, 5], false; "increasing then decreasing between 1-3 and 3-2")]
    #[test_case(vec![8, 6, 4, 4, 1], false; "no change between 4 and 4")]
    #[test_case(vec![1, 3, 6, 7, 9], true; "levels increasing by 1, 2, or 3")]
    fn test_report_is_safe(data: Vec<i32>, expected: bool) {
        // Arrange
        let report = Report::new(data);

        // Act
        let result = report.is_safe();

        // Assert
        assert_eq!(result, expected);
    }

    #[test_case(vec![7, 6, 4, 2, 1], true; "Safe without removing any level 1.")]
    #[test_case(vec![1, 2, 7, 8, 9], false; "Unsafe regardless of which level is removed 1.")]
    #[test_case(vec![9, 7, 6, 2, 1], false; "Unsafe regardless of which level is removed 2.")]
    #[test_case(vec![1, 3, 2, 4, 5], true; "Safe by removing the second level.")]
    #[test_case(vec![8, 6, 4, 4, 1], true; "Safe by removing the third level.")]
    #[test_case(vec![1, 3, 6, 7, 9], true; "Safe without removing any level 2.")]
    fn test_report_is_safe_with_tolerance(data: Vec<i32>, expected: bool) {
        // Arrange
        let report = Report::new(data);

        // Act
        let result = report.is_safe_with_tolerance();

        // Assert
        assert_eq!(result, expected);
    }
}
