//fn main() {
//    let a: [i32; 3] = [1, 2, 3];
//    let b: [i32; 5] = [1, 2, 3, 4, 5];
//    let c = [3; 5];
//    for i in a {
//        println!("{:?}", i);
//    }
//}

//Invalid Array Element Access
use std::io;
fn main() {
    let a: [u32; 5] = [1, 2, 3, 4, 5];
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Error reading line");

    let index: usize = match index.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    println!("The element at index {} is {}", index, a[index]);
}
// result
// ./array
//66

//thread 'main' panicked at array.rs:24:54:
//index out of bounds: the len is 5 but the index is 66
//note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
