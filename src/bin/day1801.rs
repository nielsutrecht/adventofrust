extern crate aoc;

use aoc::io::read_lines;
use std::collections::HashSet;

fn main() {
    let numbers: Vec<i32> = read_lines(2018, 1).iter().map(|n| n.parse::<i32>().unwrap()).collect();

    println!("{}", numbers.iter().sum::<i32>());
    println!{"{}", part2(&numbers)}
}

fn part2(numbers: &[i32]) -> String {
    let mut freq = 0;
    let mut set: HashSet<i32> = HashSet::new();

    loop {
        for i in numbers.iter() {
            freq += i;

            if !set.insert(freq){
                return freq.to_string();
            }
        }
    }
}

