//sample programs for functions in rust
fn add(x: u32) -> u32 {
    x + 8 // equivalent to return x + 8;
}
fn factorial(x: u32) -> u32 {
    if x == 1 {
        return 1;
    }
    return x * factorial(x - 1);
}
fn main() {
    let x: u32 = 5;
    let fact: u32 = factorial(x);
    println!("{}", fact);
    let y: u32 = add(2);
    println!("{}", y);
}
