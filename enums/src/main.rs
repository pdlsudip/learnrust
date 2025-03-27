use rand::{self, Rng};
use std::io;
enum Coin {
    Head,
    Tail,
}

impl Coin {
    fn flip() -> Self {
        let x: i32 = rand::rng().random_range(0..=1); //rng is seeder it seed upon the execution
                                                      //thred
        println!("{}", x);
        if x == 0 {
            return Coin::Head;
        }
        return Coin::Tail;
    }
}
enum Color {
    Red,
    Green,
    Yellow,
}
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let color = match input.trim().to_lowercase().as_str() {
        "red" => Color::Red,
        "yellow" => Color::Yellow,
        "green" => Color::Green,

        _ => {
            println!("Invalid color");
            return;
        }
    };
    match color {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Yellow => println!("Yellow"),
    }

    let result = Coin::flip();
    match result {
        Coin::Head => println!("heads"),
        Coin::Tail => println!("tails"),
    };
}
