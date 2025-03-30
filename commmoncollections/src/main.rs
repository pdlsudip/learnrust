mod erros;
mod hashmaps;
use erros::errorfn;
use hashmaps::hashmap;

#[derive(Debug)]
enum Data {
    Int(i32),
    Float(f64),
    Text(String),
}
fn main() {
    let mut vec = vec![1, 2, 3, 4, 5];

    let third = &vec[2];
    println!("{}", third);
    let third = match vec.get(2) {
        Some(&val) => val,
        None => return,
    };
    println!("{}", third);
    println!("Before squaring");
    for i in &vec {
        println!("{}", i);
    }
    for i in &mut vec {
        *i = *i * 2;
    }
    println!("After squaring");
    for i in &vec {
        println!("{}", i);
    }
    let vec1 = vec![
        Data::Int(3),
        Data::Float(2.3),
        Data::Text(String::from("Hello")),
    ];
    for i in vec1 {
        println!("{:?}", i);
    }

    let s1 = "22";
    let s2 = "sp";
    let s3 = format!("{s2} : {s1}");
    println!("{}", s3);
    hashmap();
    errorfn();
}
