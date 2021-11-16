
fn main() {
    let s1 = "Hello world";

    println!("first word of s1: {}", first(&s1));
}

// 编译器会尽可能的减轻开发者标注生命周期的负担
// 所有引用类型的参数都有独立的生命周期 'a 、'b 等。
// 如果只有一个引用型输入，它的生命周期会赋给所有输出。
// 如果有多个引用类型的参数，其中一个是 self，那么它的生命周期会赋给所有输出。
fn first(s: &str) -> &str {
    let trimmed = s.trim();
    match trimmed.find(' ') {
        None => "",
        Some(pos) => &trimmed[..pos],
    }
}
