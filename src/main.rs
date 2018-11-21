use std::env;
use std::fs;

mod hello;

use hello::print_hello;

fn main() {
    print_hello();

    let args: Vec<String> = env::args().collect();

    let guess = parse_args(&args);

    let filename = "input/2018/day01.txt";

    println!("day: {}", guess);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
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