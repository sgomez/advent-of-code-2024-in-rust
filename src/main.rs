mod day01;
mod day02;
mod day03;
mod day04;
mod utils;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day_to_run = if args.len() > 1 {
        args[1].parse::<u32>().unwrap_or(24)
    } else {
        4
    };

    match day_to_run {
        1 => day01::run(),
        2 => day02::run(),
        3 => day03::run(),
        4 => day04::run(),
        _ => eprintln!("Error: day {} not implemented.", day_to_run), // Manejo de errores
    }
}
