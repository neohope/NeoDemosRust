// 这个例子说明了，生命周期在调用过程中要保持一致
// 而且一个trait函数的实现，不能有多个生命周期的实现，只能有一个实现

trait Print {
    fn print(self);
}

// 首先我们写了一个static的实现
impl Print for &'static str {
    fn print(self) {
        println!("'static str: {}", self);
    }
}

// 如果我们再写一个a的实现，编译器会报错
// impl<'a> Print for &'a str {
//     fn print(self) {
//         println!("Arbitrary str: {}", self);
//     }
// }

// lifetime设置的是a，但s.print是static，双方不匹配，会报错
// 如果s.print是a，就可以了
// fn print_str<'a>(s: &'a str) {
//     s.print()
// }

fn main() {
    let s = "hello, world!";
    s.print();
    // print_str(s);
}
