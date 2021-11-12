/*
增加vec大小，导致内存重新分配
所以，borrow时，不可以多个读
 */

use std::mem;

fn main() {
    // capacity 是 1, len 是 0
    let mut v = vec![1];

    // 我们先打印 heap 地址，然后看看添加内容是否会导致堆重分配
    println!("origin heap start: {:p}", &v[0] as *const i32);

    // 扩展vec
    extend_vec(&mut v);

    // heap 地址改变了！
    // 这就是为什么可变引用和不可变引用不能共存的原因
    println!("new heap start: {:p}", &v[0] as *const i32);
    print_vec("v", v);
}

fn extend_vec(v: &mut Vec<i32>) {
    // Vec<T> 堆内存里 T 的个数是指数增长的，我们让它恰好 push 33 个元素
    // capacity 会变成 64
    (2..34).into_iter().for_each(|i| v.push(i));
}

fn print_vec<T>(name: &str, data: Vec<T>) {
    let p: [usize; 3] = unsafe { mem::transmute(data) };
    // 打印 Vec<T> 的堆地址，capacity，len
    println!("{}: mem location 0x{:x}, size {}, first element {}", name, p[0], p[1], p[2]);
}
