use std::collections::HashMap;
use std::fs;

pub(crate) fn run() {
    let filename = "data/day06/input.txt";
    let mut game = Game::from_file(filename);
    let result_part_1 = game.play();
    println!("Result of Day 06, Part 1 is {}", result_part_1);
    let mut game = Game::from_file(filename);
    let result_part_2 = game.play_loop();
    println!("Result of Day 06, Part 2 is {}", result_part_2);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Game {
    position: (usize, usize),
    direction: Direction,
    map: HashMap<(usize, usize), char>,
    ended: bool,
    ended_by_loop: bool,
}

impl Game {
    fn from_game(game: &Game) -> Self {
        game.clone()
    }

    fn from_file(filename: &str) -> Game {
        let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

        let mut position = (0, 0);
        let mut direction = Direction::Up;
        let mut map: HashMap<(usize, usize), char> = HashMap::new();

        for (row, line) in contents.lines().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if let Some(dir) = match ch {
                    '^' => Some(Direction::Up),
                    '>' => Some(Direction::Right),
                    'v' => Some(Direction::Down),
                    '<' => Some(Direction::Left),
                    _ => None,
                } {
                    position = (row, col);
                    direction = dir;
                    map.insert((row, col), 'X'); // Reemplazar símbolo del personaje
                } else {
                    map.insert((row, col), ch);
                }
            }
        }

        Self {
            position,
            direction,
            map,
            ended: false,
            ended_by_loop: false,
        }
    }

    #[allow(dead_code)]
    fn print_grid(&self) {
        let (max_row, max_col) = self
            .map
            .keys()
            .fold((0, 0), |(max_row, max_col), (row, col)| {
                (max_row.max(*row), max_col.max(*col))
            });

        for row in 0..=max_row {
            for col in 0..=max_col {
                // Imprimir el valor de la posición en el mapa, o un espacio si no existe
                let symbol = self.map.get(&(row, col)).unwrap_or(&' ');
                print!("{}", symbol);
            }
            println!();
        }
        println!();
        println!();
    }

    fn next_position(&self) -> (usize, usize) {
        let (row, col) = self.position;
        let next_position = match self.direction {
            Direction::Up => (row.wrapping_sub(1), col),
            Direction::Down => (row + 1, col),
            Direction::Left => (row, col.wrapping_sub(1)),
            Direction::Right => (row, col + 1),
        };

        next_position
    }

    fn turn_until_free_path(&mut self) {
        let next_position = self.next_position();

        match self.map.get(&next_position) {
            Some('#') => {
                self.direction = self.direction.turn();
                self.turn_until_free_path();
            }
            _ => {}
        }
    }

    fn next(&mut self) {
        if self.ended {
            return;
        }

        let next_position = self.next_position();

        match self.map.get_mut(&next_position) {
            Some('#') => {
                if self.map.get(&self.position) == Some(&'+') {
                    self.ended = true;
                    self.ended_by_loop = true;
                    return;
                }
                self.map.entry(self.position).and_modify(|c| *c = '+');
                self.turn_until_free_path();
            }
            Some('O') => {
                self.direction = self.direction.turn();
            }
            Some('+') | Some(_) => {
                self.position = next_position;

                if let Some(cell) = self.map.get_mut(&next_position) {
                    if *cell == '.' {
                        *cell = 'X';
                    }
                }
            }
            None => {
                self.ended = true;
            }
        }
    }

    fn play(&mut self) -> i32 {
        while !self.ended {
            self.next();
        }

        self.map
            .iter()
            .filter(|(_, v)| **v == 'X' || **v == '+')
            .count() as i32
    }

    fn play_loop(&mut self) -> i32 {
        let mut count = 0;

        for (pos, value) in self.map.iter() {
            if *value != '.' {
                continue;
            }

            let mut loop_game = Game::from_game(self);

            loop_game.map.entry(*pos).and_modify(|c| *c = 'O');

            loop_game.play();

            if loop_game.ended_by_loop {
                count += 1;
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use crate::day06::{Direction, Game};

    #[test]
    fn test_create_game() {
        // Arrange
        let game = Game::from_file("data/day06/example.txt");
        // Act
        // Assert
        assert_eq!(game.position, (6, 4));
        assert_eq!(game.direction, Direction::Up);
        assert_eq!(game.map.get(&(6, 4)), Some(&'X'));
    }

    #[test]
    fn test_run_move_up() {
        // Arrange
        let mut game = Game::from_file("data/day06/example.txt");
        // Act
        game.next();
        // Assert
        assert_eq!(game.position, (5, 4));
        assert_eq!(game.direction, Direction::Up);
        assert_eq!(game.map.get(&(5, 4)), Some(&'X'));
    }

    #[test]
    fn test_run_turn_right() {
        // Arrange
        let mut game = Game::from_file("data/day06/example.txt");
        // Act
        game.next();
        game.next();
        game.next();
        game.next();
        game.next();
        game.next();
        // Assert
        assert_eq!(game.position, (1, 4));
        assert_eq!(game.direction, Direction::Right);
    }

    #[test]
    fn test_run_game() {
        // Arrange
        let mut game = Game::from_file("data/day06/example.txt");
        // Act
        let result = game.play();
        // Assert
        assert_eq!(result, 41);
    }

    #[test]
    fn test_detect_loop() {
        // Arrange
        let mut game = Game::from_file("data/day06/example.txt");
        game.map.entry((6, 3)).and_modify(|c| *c = '#');
        // Act
        game.play();
        // Assert
        assert_eq!(game.ended_by_loop, true);
    }

    #[test]
    fn test_run_game_loop() {
        // Arrange
        let mut game = Game::from_file("data/day06/example.txt");
        // Act
        let result = game.play_loop();
        // Assert
        assert_eq!(result, 6);
    }
}
