use std::mem::{align_of, size_of};

#[allow(dead_code)]
struct S1 {
    a: u8,        //8
    b: u16,       //16
    c: u8,        //8
}

#[allow(dead_code)]
struct S2 {
    a: u8,        //8
    c: u8,        //8
    b: u16,       //16
}

fn main() {
    // Rust不仅会内存对齐，还会自动优化内存
    println!("sizeof S1: {}, S2: {}", size_of::<S1>(), size_of::<S2>());
    // ABI要求最小大小，sizeof必须是它的倍数
    println!("alignof S1: {}, S2: {}", align_of::<S1>(), align_of::<S2>());
}
