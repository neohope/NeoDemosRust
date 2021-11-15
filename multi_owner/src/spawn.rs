use std::{sync::Arc, thread};

fn test01(){
    let arr = vec![1,2,3,4,5];

    // 必须用move语法，将arr所有权交给join线程
    let handler = thread::spawn(move || {
      println!("{:?}", arr);
    });

    // 如果再增加下面一行，就无法通过编译了，因为所有权已经move了
    // println!("{:?}", arr);

    handler.join().unwrap();
}

fn test02(){
    let str = Arc::new(String::from("hello"));
    let str_clone = Arc::clone(&str);

    // 必须用move语法，将arr所有权交给join线程
    let handler = thread::spawn( move || {
      println!("{:?}", str_clone);
    });

    // 通过Arc就可以在两个线程下只读了
    println!("{:?}", str);

    handler.join().unwrap();
}

fn main() {
    test01();
    test02();
}