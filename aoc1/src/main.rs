use std::fs;
fn main() {
    let mut veca: Vec<i64> = Vec::new();
    let mut vecb: Vec<i64> = Vec::new();
    let inputs = fs::read_to_string("../../a.txt").expect("Failed to read file");

    for line in inputs.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        veca.push(parts[0].parse().expect("Invalid number"));
        vecb.push(parts[1].parse().expect("Invalid number"));
    }

    veca.sort();
    vecb.sort();
    let mut sum: i64 = 0;
    for i in 0..veca.len().min(vecb.len()) {
        sum += (veca[i] - vecb[i]).abs();
    }
    println!("{}", sum);
}
