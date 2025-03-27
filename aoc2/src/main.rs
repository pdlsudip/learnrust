use std::{fs, io::BufRead};
fn main() {
    let data = fs::read("./inputs.txt").expect("file doesnot exist");
    let mut counter: u32 = 0;
    for lines in data.lines() {
        for (_, data) in lines.iter().enumerate() {
            let xvec: Result<Vec<i32>, _> =
                data.split_whitespace().map(|s| s.parse::<i32>()).collect();
            let mvec = match xvec {
                Ok(value) => value,
                Err(_) => return,
            };
            let check = check_safe(&mvec);
            if check == true {
                counter += 1;
            }
        }
    }
    println!("count: {}", counter);
}

fn check_safe(myvec: &Vec<i32>) -> bool {
    if myvec.len() < 2 {
        return true;
    }
    let first_diff = myvec[1] - myvec[0];
    if first_diff == 0 || first_diff.abs() > 3 {
        return false;
    }
    let increasing = first_diff > 0;

    for i in 0..myvec.len() - 1 {
        let diff = myvec[i + 1] - myvec[i];
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }
        if (increasing && diff <= 0) || (!increasing && diff >= 0) {
            return false;
        }
    }
    true
}
