pub mod io;
pub mod convert;
pub mod math;

pub trait Day {
    fn new() -> Self;
    fn part1(&self) -> String;
    fn part2(&self) -> String;
}