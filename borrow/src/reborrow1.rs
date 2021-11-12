fn main() {
    let mut a = vec![1, 2, 3, 4];

    let b = &mut a;
    
    println!("addr of the ref b: {:p}", &b);
    
    println!("sum of a: {}", sum(b));

    // borrow后，用了a就不能再用b
    // println!("{:?}", a);
    println!("{:?}", b);

    println!("{:?}", a);
}

fn sum(v: &mut Vec<i32>) -> i32 {
    println!("addr of the ref v: {:p}", &v);
    v.iter().sum()
}
