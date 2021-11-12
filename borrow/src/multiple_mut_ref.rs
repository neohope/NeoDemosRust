#[allow(unused_variables, unused_mut)]
fn main() {
    let mut data = vec![1, 2, 3];
    println!("data[0]: {:p}", &data[0]);

    //加上这句，就造成了同时有可变引用和只读引用
    //let data1 = vec![&data[0]];

    // Rust 下，不能同时拥有可变引用和只读引用
    for i in 0..100 {
        data.push(i);
    }

    println!("data[0]: {:p}", &data[0]);

    //而下面只有只读引用就可以了
    let data1 = vec![&data[0]];
    println!("boxed: {:p}", &data1);
}
