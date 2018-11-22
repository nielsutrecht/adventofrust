extern crate aoc;

use std::env;
use aoc::io::read_file;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day = parse_args(&args);

    let contents = read_file(2018, day as u8);

    println!("Day {:02}:\n{}", day, contents);
}

fn parse_args(args: &[String]) -> u32 {
    if args.len() > 1 {
        match args[1].trim().parse() {
            Ok(num) => return num,
            Err(_) => return 0,
        };
    } else {
        return 0
    };
}