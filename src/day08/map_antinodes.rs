use crate::day08::coords::Coords;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone)]
pub struct MapAntinodes {
    antennas: HashMap<Coords, char>,
    antinodes: HashMap<Coords, char>,
}

impl MapAntinodes {
    pub(crate) fn from_file(filename: &str) -> MapAntinodes {
        let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
        let mut antennas: HashMap<Coords, char> = HashMap::new();
        let mut antinodes: HashMap<Coords, char> = HashMap::new();
        for (row, line) in contents.lines().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                let row_i32 = row as i32;
                let col_i32 = col as i32;

                antennas.insert(Coords::new(row_i32, col_i32), ch);
                antinodes.insert(Coords::new(row_i32, col_i32), '.');
            }
        }

        Self {
            antennas,
            antinodes,
        }
    }

    pub(crate) fn analyze(&mut self) -> i32 {
        let antennas_coords: Vec<Coords> = self.antennas.keys().cloned().collect();
        let num_antennas = antennas_coords.len();

        for i in 0..num_antennas - 1 {
            let first_antenna = antennas_coords.get(i).unwrap();

            for j in i + 1..num_antennas {
                let second_antenna = antennas_coords.get(j).unwrap();

                self.compare_antennas(first_antenna, second_antenna);
            }
        }

        let count = self.antinodes.iter().filter(|&(_, c)| *c == '#').count();

        count as i32
    }

    fn compare_antennas(&mut self, coords_first: &Coords, coords_second: &Coords) {
        if let (Some(first), Some(second)) = (
            self.antennas.get(coords_first),
            self.antennas.get(coords_second),
        ) {
            if first == second && first != &'.' {
                let (first_antinode, second_antinode) = coords_first.antinodes(coords_second);
                if let Some(value) = self.antinodes.get_mut(&first_antinode) {
                    *value = '#'
                }
                if let Some(value) = self.antinodes.get_mut(&second_antinode) {
                    *value = '#'
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_part() {
        // Arrange
        let mut map = MapAntinodes::from_file("data/day08/example.txt");
        // Act
        let result = map.analyze();
        // Assert
        assert_eq!(result, 14);
    }
}
