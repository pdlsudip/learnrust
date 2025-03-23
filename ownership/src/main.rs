// 1. Each value in Rust has an owner.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.
// integers float char boolean tuple of variable with copy traits have copy traits so there values
// doesnot move but copy as there are place in stack unlike String
fn takes_ownership(s: String) {
    println!("Some string : {}", s);
}
fn makes_copy(a: i32) {
    println!("{}", a);
}
fn change(s: &mut String) {
    s.push_str(", world!");
}
fn main() {
    let _s: &str = "Hello from rust"; // string stored in stack it has fixed type
    let mut s1: String = String::from("Hello from rust"); // stored in heap don't have a fixed size
    s1.push_str(", ownership model"); // so we can push after but cannot with string literals
    let s2 = s1; // if we give reference to s2= &s1; s1 will not be out of scope
                 // let s2 = s1.clone(); // this makes copy of actual but this is memory expensive
    println!("{s2}"); // works but now s1 is out of scope [point no 2]
                      //println!("{s1}");
    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}"); //this works but there is a contradiction  [ but simple data
                                  //types are stored in stack so it is easy to copy and not memory expensive as stored in heap]

    //ownership in function
    let string: String = String::from("Hello, World!");
    takes_ownership(string); // now this function takes the ownership of this string now it cannot
                             // be used as String doesnot have copy trait
    let int: i32 = 3;
    makes_copy(int); //copy is passed into function
    println!("{}", int); // so int is in the scope

    //borrowing eg
    let mut string1 = String::from("hello");
    change(&mut string1);
    println!("{}", string1);
}
