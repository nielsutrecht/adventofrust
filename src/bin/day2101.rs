extern crate aoc;

use aoc::io::{read_buf, read_file, read_lines};
use std::collections::HashSet;
use std::fmt::format;
use std::io::BufRead;
use aoc::convert::string_to_digits;
use aoc::Day;

struct Day2101 {
    digits: Vec<u32>,
}

impl Day for Day2101 {
    fn new() -> Day2101 {
        let digits = read_buf(2021, 1).lines().map(|l| l.unwrap().parse::<u32>().unwrap())
            .collect();

        return Day2101 { digits };
    }

    fn part1(&self) -> String {
        return self.digits.windows(2).filter(|w| w[0] < w[1]).count().to_string();
    }

    fn part2(&self) -> String {
        return self.digits.windows(4).filter(|w| w[0] < w[3]).count().to_string();
    }
}

fn main() {
    let day = Day2101::new();

    println!("{}", day.part1());
    println!("{}", day.part2());
}

