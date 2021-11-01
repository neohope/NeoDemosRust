enum MyOption<T> {
    Some(T),
    None,
}

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