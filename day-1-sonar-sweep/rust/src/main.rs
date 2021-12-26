use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn count_increased_depths() -> u32 {
    let mut previous_value: u32 = std::u32::MAX;
    let mut counter = 0;
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(actual_line) = line {
                let current_value: u32 = actual_line.parse().unwrap();
                if previous_value < current_value {
                    counter += 1;
                }
                previous_value = current_value;
            }
        }
    }
    return counter;
}

fn main() {
    println!("Counter of descents: {}", count_increased_depths());
}
