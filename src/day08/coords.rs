#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Coords {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

impl Coords {
    pub fn new(x: i32, y: i32) -> Coords {
        Self { x, y }
    }

    pub fn antinodes(&self, opposite: &Coords) -> (Coords, Coords) {
        let dif_x = self.x - opposite.x;
        let dif_y = self.y - opposite.y;

        (
            Coords::new(self.x + dif_x, self.y + dif_y),
            Coords::new(opposite.x - dif_x, opposite.y - dif_y),
        )
    }

    pub fn harmonics(&self, opposite: &Coords, max_x: i32, max_y: i32) -> Vec<Coords> {
        let mut points = Vec::new();

        let delta_x = opposite.x - self.x;
        let delta_y = opposite.y - self.y;

        let step_x = delta_x;
        let step_y = delta_y;

        let mut current_x = self.x;
        let mut current_y = self.y;

        loop {
            points.push(Coords::new(current_x, current_y));

            current_x += step_x;
            current_y += step_y;

            if current_x >= max_x || current_y >= max_y || current_x < 0 || current_y < 0 {
                break;
            }
        }

        current_x = self.x - step_x;
        current_y = self.y - step_y;

        loop {
            points.push(Coords::new(current_x, current_y));

            current_x -= step_x;
            current_y -= step_y;

            if current_x >= max_x || current_y >= max_y || current_x < 0 || current_y < 0 {
                break;
            }
        }

        points
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(Coords::new(3, 4), Coords::new(5, 6), (Coords::new(1, 2), Coords::new(7, 8)))]
    #[test_case(Coords::new(5, 6), Coords::new(3, 4), (Coords::new(7, 8), Coords::new(1, 2)))]
    #[test_case(Coords::new(4, 3), Coords::new(6, 5), (Coords::new(2, 1), Coords::new(8, 7)))]
    #[test_case(Coords::new(6, 5), Coords::new(4, 3), (Coords::new(8, 7), Coords::new(2, 1)))]
    fn test_create_coords(first: Coords, second: Coords, expected: (Coords, Coords)) {
        // Arrange
        // Act
        let result = first.antinodes(&second);
        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn test_create_harmonics() {
        // Arrange
        let first = Coords::new(2, 1);
        let second = Coords::new(4, 2);
        // Act
        let mut result = first.harmonics(&second, 10, 10);
        result.sort_by(|a, b| a.x.cmp(&b.x).then(a.y.cmp(&b.y)));

        // Assert
        assert_eq!(
            result,
            vec![
                Coords::new(0, 0),
                Coords::new(2, 1),
                Coords::new(4, 2),
                Coords::new(6, 3),
                Coords::new(8, 4)
            ]
        )
    }
}
