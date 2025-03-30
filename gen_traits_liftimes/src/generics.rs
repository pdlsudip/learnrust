#[derive(Debug)]
struct Points<T> {
    x: T,
    y: T,
}
struct Pointss<T, U> {
    x: T,
    y: U,
}
impl<T> Points<T> {
    fn set(&mut self, x: T, y: T) {
        self.x = x;
        self.y = y;
    }
}
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
pub fn genericfn() {
    let arr: [i32; 5] = [2, 3, 4, 5, 6];
    let result = largest(&arr);
    println!("{}", result);
    let mut s = Points { x: 3, y: 5 };
    println!("{:?} and {:?}", s.x, s.y);
    Points::set(&mut s, 3, 4);
}
