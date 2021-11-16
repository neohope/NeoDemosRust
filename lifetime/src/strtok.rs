// 返回值的生命周期，和&str一致，与&mut不一致，所以返回值生命周期应该为a
// 同时b的生命周期，编译器就可以自己推断出来了，所以下面生命周期中，其实可以省略b
// pub fn strtok<'b, 'a>(s: &'b mut &'a str, delimiter: char) -> &'a str {
pub fn strtok<'a>(s: &mut &'a str, delimiter: char) -> &'a str {
    if let Some(i) = s.find(delimiter) {
        let prefix = &s[..i];
        let suffix = &s[(i + delimiter.len_utf8())..];
        *s = suffix;
        prefix
    } else {
        let prefix = *s;
        *s = "";
        prefix
    }
}

// 但如果改成b的生命周期
// pub fn strtok<'b>(s: &'b mut &str, delimiter: char) -> &'b str {}

// 相当于把对字符串的可变引用的生命周期和返回值的生命周期关联起来了，hello 得到了 strtok 的返回值
// 所以从编译器看来，&mut s1 的生命周期并没有结束
// let hello = strtok(&mut s1, ' ');

// 所以这里发生了 &mut 和 & 同时存在的局面。
// println!("prefix is: {}, suffix is: {}, str is: {}", hello, s1, str);

fn main() {
    let str = "hello world".to_owned();
    let mut s1 = str.as_str();
    let hello = strtok(&mut s1, ' ');
    println!("prefix is: {}, suffix is: {}, str is: {}", hello, s1, str);
}
