use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Welcome to guessing game");
    println!("Enter your Guess");
    let secret_number = rand::rng().random_range(1..=100); //rng() seed upon execution thread and random_range take a range of numbers
    println!("Secret number is : {}", secret_number);
    loop {
        let mut guess = String::new(); //Initiliaze a string
        io::stdin()
            .read_line(&mut guess)
            .expect("Unable to read Line");
        let guess: u32 = match guess.trim().parse() {
            // kinda error handling
            Ok(num) => num,
            Err(_) => {
                println!("Enter a number");
                continue;
            }
        };
        match guess.cmp(&secret_number) {
            //inbuilt cmp trait(maybe) for comparison
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        if guess == 0 {
            break;
        }
    }
}
