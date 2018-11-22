use std::fs::*;
use std::io::prelude::*;
use std::io::BufReader;


pub fn read_file(year: u32, day: u8) -> String {
    return read_to_string(file_name(year, day))
        .expect("Something went wrong reading the file");
}

pub fn read_lines(year: u32, day: u8) -> Vec<String> {
    let file = File::open(file_name(year, day)).expect("Something went wrong reading the file");
    let buf = BufReader::new(file);

    return buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();
}

fn file_name(year: u32, day: u8) -> String {
    return format!("input/{:04}/day{:02}.txt", year, day);
}

#[cfg(test)]
mod tests {
    #[test]
    fn file_name() {
        assert_eq!("input/2018/day01.txt", super::file_name(2017, 1));
    }

    #[test]
    fn read_file() {
        let s = super::read_file(2017, 1);
        assert_eq!(true, s.starts_with("9514"))
    }

    #[test]
    fn read_lines() {
        let s = super::read_lines(2017, 2);
        assert_eq!(16, s.len());
        assert_eq!(true, s[1].starts_with("961\t98"));
    }
}