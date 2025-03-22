fn main() {
    for number in (1..4).rev() {
        // rev the range with .rev
        println!("{number}!");
    }
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    for i in a {
        println!("{i}!");
    }
    'this: loop {
        println!("infinte");
        break 'this; // this is label this is written with '
    }
}
