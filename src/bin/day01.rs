extern crate aoc;

use aoc::io::read_file;
use aoc::convert::string_to_digits;

fn main() {
    let contents = read_file(2018, 1);
    let digits = string_to_digits(contents);

    part_a(&digits);
    part_b(&digits);
}

fn part_a(digits: &[u32]) {
    let mut sum = 0;

    for i in 0..digits.len() {
        let j = if i == digits.len() - 1 { 0 } else { i + 1 };

        if digits[i] == digits[j] {
            sum += digits[i];
        }
    }

    println!("{}", sum);
}

fn part_b(digits: &[u32]) {
    let mut sum = 0;

    for i in 0..digits.len() {
        let j = i + digits.len() / 2;
        let j = if j >= digits.len() { j - digits.len() } else { j };

        if digits[i] == digits[j] {
            sum += digits[i];
        }
    }

    println!("{}", sum);
}