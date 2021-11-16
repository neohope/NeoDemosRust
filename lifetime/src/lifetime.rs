fn main() {
    let s1 = String::from("Lindsey");
    let s2 = String::from("Rosie");

    let result = max(&s1, &s2);

    println!("bigger one: {}", result);

    let result = get_max(&s1);
    println!("bigger one: {}", result);
}

// 只有一个参数，编译器可以分析生命周期
fn get_max(s1: &str) -> &str {
    max(s1, "Cynthia")
}

// 多个参数，生命周期可能并不一致，编译器无法分析返回值生命周期
// 需要加上生命周期标注，告诉编译器这些引用间生命周期的约束
fn max<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1 > s2 {
        s1
    } else {
        s2
    }
}
