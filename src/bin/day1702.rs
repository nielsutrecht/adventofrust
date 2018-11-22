extern crate aoc;

use aoc::io::read_lines;
use aoc::convert::string_vec_to_ints;
use aoc::*;

use std::cmp;


struct Day1702 {
    sheet: Vec<Vec<i32>>
}

impl Day1702 {
    fn lowest_highest(vec: &[i32]) -> (i32, i32) {
        let mut lowest = std::i32::MAX;
        let mut highest = std::i32::MIN;

        for i in vec {
            lowest = cmp::min(lowest, *i);
            highest = cmp::max(highest, *i);
        }

        return (lowest, highest);
    }

    fn evenly_divisible(vec: &[i32]) -> i32 {
        for i in vec {
            for j in vec {
                if j != i && ( j % i == 0 || i % j == 0 ) {
                    return cmp::max(i, j) / cmp::min(i, j)
                }
            }
        }

        return 0;
    }
}

impl Day for Day1702 {
    fn new() -> Day1702 {
        let contents = read_lines(2017, 2);
        let sheet = string_vec_to_ints(contents);

        return Day1702 { sheet };
    }

    fn part1(&self) -> String {
        let x : i32 = self.sheet.clone()
            .into_iter().map(|v| Day1702::lowest_highest(v.as_slice()))
            .map(| t| t.1 - t.0)
            .sum();

        return x.to_string();
    }

    fn part2(&self) -> String {
        let x : i32 = self.sheet.clone()
            .into_iter().map(|v| Day1702::evenly_divisible(v.as_slice()))
            .sum();

        return x.to_string();
    }
}

fn main() {
    let day = Day1702::new();

    println!("{}", day.part1());
    println!("{}", day.part2());
}

#[cfg(test)]
mod tests {
    use aoc::*;

    #[test]
    fn parts() {
        let day = super::Day1702::new();

        assert_eq!("21845", day.part1());
        assert_eq!("191", day.part2());
    }
}