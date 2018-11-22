extern crate aoc;

use aoc::io::read_file;
use aoc::convert::string_to_digits;
use aoc::*;

struct Day1701 {
    digits: Vec<u32>
}

impl Day1701 {
    fn solve<F>(digits: &Vec<u32>, f: F) -> String where F: Fn(usize) -> usize {
        let mut sum = 0;

        for i in 0..digits.len() {
            if digits[i] == digits[f(i)] {
                sum += digits[i];
            }
        }

        return sum.to_string();
    }
}

impl Day for Day1701 {
    fn new() -> Day1701 {
        let contents = read_file(2017, 1);
        let digits = string_to_digits(contents);

        return Day1701 { digits };
    }

    fn part1(&self) -> String {
        let part_1 = |i: usize| -> usize { if i == self.digits.len() - 1 { 0 } else { i + 1 } };

        return Day1701::solve(&self.digits, part_1)
    }

    fn part2(&self) -> String {
        let part_2 = |i: usize| -> usize {
            let j = i + self.digits.len() / 2;
            if j >= self.digits.len() { j - self.digits.len() } else { j }
        };

        return Day1701::solve(&self.digits, part_2)
    }
}

fn main() {
    let day = Day1701::new();

    println!("{}", day.part1());
    println!("{}", day.part2());
}

#[cfg(test)]
mod tests {
    use aoc::*;

    #[test]
    fn parts() {
        let day = super::Day1701::new();

        assert_eq!("1341", day.part1());
        assert_eq!("1348", day.part2());
    }
}