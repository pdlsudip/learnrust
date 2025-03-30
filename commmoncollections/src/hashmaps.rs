use std::collections::HashMap;
pub fn hashmap() {
    let mut map = HashMap::new();
    let text = "Hello from Rust and uses of Hashmaps from Rust";
    map.insert(String::from("Red"), 20);
    map.insert(String::from("Blue"), 20);
    for (k, v) in map {
        println!("Team: {}, Score:{}", k, v);
        println!();
    }
    let mut map2 = HashMap::new();
    for word in text.split_whitespace() {
        map2.entry(word).or_insert(0);
    }
    println!("{map2:?}");
}
