mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod utils;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day_to_run = if args.len() > 1 {
        args[1].parse::<u32>().unwrap_or(24)
    } else {
        10
    };

    match day_to_run {
        1 => day01::run(),
        2 => day02::run(),
        3 => day03::run(),
        4 => day04::run(),
        5 => day05::run(),
        6 => day06::run(),
        7 => day07::run(),
        8 => day08::run(),
        9 => day09::run(),
        10 => day10::run(),
        _ => eprintln!("Error: day {} not implemented.", day_to_run), // Manejo de errores
    }
}
