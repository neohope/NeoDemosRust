// enum类型，支持泛型，支持模式匹配

#[allow(dead_code)]
#[derive(Debug)]
enum MyOption<T> {
    Some(T),
    None,
}

#[allow(dead_code)]
#[derive(Debug)]
enum Status {
    Ok = 0,
    BadName = 1,
    NotFound = 2,
    Internal = 3,
}

fn main() {
    let opt = MyOption::Some("hello");
    let status = Status::NotFound;
    println!("opt is {:?}, status is: {:?}", opt, status);
}