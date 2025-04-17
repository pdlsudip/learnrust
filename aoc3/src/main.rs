use regex::Regex;
use std::fs;

fn main() {
    let contents = fs::read_to_string("./input.txt").unwrap();
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut sum = 0;
    // Iterate over all matches and print the captured numbers
    for caps in re.captures_iter(&contents) {
        let num1 = caps.get(1).map_or("", |m| m.as_str());
        let num2 = caps.get(2).map_or("", |m| m.as_str());
        let num1: i32 = num1.trim().parse().unwrap();
        let num2: i32 = num2.trim().parse().unwrap();
        sum += num1 * num2;
    }
    println!("{}", sum);
}
