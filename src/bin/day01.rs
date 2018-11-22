extern crate aoc;

use aoc::io::read_file;
use aoc::convert::string_to_digits;

fn main() {
    let contents = read_file(2018, 1);
    let digits = string_to_digits(contents);

    let part_1 = |i: usize| -> usize { if i == digits.len() - 1 { 0 } else { i + 1 } };
    let part_2 = |i: usize| -> usize {
        let j = i + digits.len() / 2;
        if j >= digits.len() { j - digits.len() } else { j }
    };

    solve(&digits, part_1);
    solve(&digits, part_2);
}

fn solve<F>(digits: &[u32], f: F) where F: Fn(usize) -> usize {
    let mut sum = 0;

    for i in 0..digits.len() {
        if digits[i] == digits[f(i)] {
            sum += digits[i];
        }
    }

    println!("{}", sum);
}