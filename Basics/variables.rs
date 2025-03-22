//8-bit	i8	u8
//16-bit	i16	u16
//32-bit	i32	u32
//64-bit	i64	u64
//128-bit	i128	u128
//f32
//f64
fn main() {
    let x: i32 = 4;
    let mut y: i32; //by default variables are not mutable so if we want to change them we
                    //need to explicitely say mut
    y = x;
    y += 1;
    println!("{}", y);
    let f: f32 = 4.2;
    let f2: f64 = 4.0;
    println!("{}", f); // doesnot five 4.0 if f = 4.0
    println!("{:?}", f2); // five 4.0 also
                          // tuples
    let tuple: (i32, f32) = (1, 1.3);
    println!("{:?}", tuple.0); // 1
    println!("{:?}", tuple.1); // 1.3
}
