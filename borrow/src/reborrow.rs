fn main() {
    let mut r0 = 42;
    println!("r0 : {:p}", &r0);

    let r1 = &mut r0;

    // 不能两次borrow可变借用
    // let r2 = &mut r0;

    // &r0 不可以，这样同时有读写
    // let r2 = &r0;

    // reborrow 可以通过
    let r2 = &*r1;

    // borrow 这里可以用
    println!("r2: {}", r2);

    println!("r1: {:p}", &r1);
    println!("r2: {:p}", &r2);

    // 改变r1，后面不能再用r2
    *r1 += 1;

    // cannot assign to `*r2`, which is behind a `&` reference
    // *r2 += 1;

    // 不可以，同时有读写
    // println!("r0: {}", r0);

    println!("r1: {}", r1);

    // borrow 到这里就不能用了
    // println!("r2: {}", r2);
}
