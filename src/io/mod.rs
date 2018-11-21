use std::fs;

pub fn read_file(year: u32, day: u8) -> String {
    let filename = format!("input/{:04}/day{:02}.txt", year, day);
    return fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
}