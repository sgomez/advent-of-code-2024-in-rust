use crate::{utils::line_integer_parser::parse_levels, utils::read_lines};
use std::collections::{HashMap, HashSet};

type Coord = (isize, isize);

#[derive(Debug)]
pub struct TrailHead {
    map: HashMap<Coord, i32>,
}

impl TrailHead {
    pub(crate) fn from_file(filename: &str) -> TrailHead {
        let mut map: HashMap<Coord, i32> = HashMap::new();

        if let Ok(lines) = read_lines(filename) {
            for (row_index, line) in lines.flatten().enumerate() {
                if let Ok((_, parsed_line)) = parse_levels(&line) {
                    for (col_index, cell) in parsed_line.into_iter().enumerate() {
                        if let Some(value) = cell {
                            map.insert((row_index as isize, col_index as isize), value);
                        }
                    }
                }
            }
        }

        TrailHead { map }
    }

    pub fn count_all_trails(&self) -> i32 {
        self.get_starts()
            .iter()
            .map(|&pos| self.find_paths(pos) as i32)
            .sum()
    }

    pub fn count_all_multiple_trails(&self) -> i32 {
        self.get_starts()
            .iter()
            .map(|&pos| self.find_multiple_paths(pos) as i32)
            .sum()
    }

    fn find_paths(&self, from: &Coord) -> usize {
        fn backtracking(
            map: &HashMap<Coord, i32>,
            pos: &Coord,
            visited: &mut HashSet<Coord>,
            trailhead: &TrailHead,
        ) {
            if visited.contains(pos) {
                return;
            }

            visited.insert(*pos);

            if let Some(level) = map.get(pos) {
                if *level == 9 {
                    return;
                }

                for next_pos in trailhead.find_next_level(pos) {
                    backtracking(map, &next_pos, visited, trailhead);
                }
            }
        }

        let mut visited: HashSet<Coord> = HashSet::new();
        backtracking(&self.map, from, &mut visited, self);

        visited
            .iter()
            .filter(|&&pos| self.map.get(&pos) == Some(&9))
            .count()
    }

    fn find_multiple_paths(&self, from: &Coord) -> i32 {
        fn backtracking(
            map: &HashMap<Coord, i32>,
            pos: &Coord,
            count: &mut i32,
            trailhead: &TrailHead,
        ) {
            if let Some(level) = map.get(pos) {
                if *level == 9 {
                    *count += 1;
                    return;
                }
            } else {
                return;
            }

            trailhead
                .find_next_level(pos)
                .iter()
                .for_each(|next_pos| backtracking(map, next_pos, count, trailhead));
        }

        let mut count = 0;
        backtracking(&self.map, from, &mut count, self);

        count
    }

    fn get_starts(&self) -> Vec<&Coord> {
        self.map
            .iter()
            .filter_map(
                |(coords, level)| {
                    if *level == 0 {
                        Some(coords)
                    } else {
                        None
                    }
                },
            )
            .collect()
    }

    fn find_next_level(&self, from: &Coord) -> Vec<Coord> {
        let level = self.map.get(from).unwrap() + 1;
        let directions = [(-1, 0), (0, -1), (1, 0), (0, 1)];

        directions
            .iter()
            .filter_map(|(dx, dy)| {
                let new_coord = (from.0 + dx, from.1 + dy);

                if let Some(&neighbor_level) = self.map.get(&new_coord) {
                    if neighbor_level == level {
                        return Some(new_coord);
                    }
                }

                None
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_trail_head() {
        // Arrange
        let trail_head = TrailHead::from_file("data/day10/example01.txt");
        // Act
        // Assert
        assert_eq!(trail_head.map.get(&(0, 0)), Some(&0));
    }

    #[test]
    fn test_create_trail_head_02() {
        // Arrange
        let trail_head = TrailHead::from_file("data/day10/example02.txt");
        // Act
        // Assert
        assert_eq!(trail_head.map.get(&(0, 0)), None);
        assert_eq!(trail_head.map.get(&(0, 3)), Some(&0));
    }

    #[test]
    fn test_find_inits() {
        // Arrange
        let trail_head = TrailHead::from_file("data/day10/example01.txt");
        // Act
        let starts = trail_head.get_starts();
        // Assert
        assert_eq!(starts.iter().count(), 1);
    }

    #[test]
    fn test_find_inits_05() {
        // Arrange
        let trail_head = TrailHead::from_file("data/day10/example05.txt");
        // Act
        let starts = trail_head.get_starts();
        // Assert
        assert_eq!(starts.iter().count(), 9);
    }

    #[test]
    fn test_find_next_level() {
        // Arrange
        let trail_head = TrailHead::from_file("data/day10/example01.txt");
        // Act
        let paths = trail_head.find_next_level(&(0, 2));
        // Assert
        assert_eq!(paths, vec![(1, 2), (0, 3)]);
    }

    #[test]
    fn test_count_paths() {
        // Arrange
        let trail_head = TrailHead::from_file("data/day10/example03.txt");
        // Act
        let paths = trail_head.count_all_trails();
        // Assert
        assert_eq!(paths, 4);
    }
}
