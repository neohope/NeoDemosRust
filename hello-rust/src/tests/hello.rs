// 单元测试

fn main() {
    println!("Hello, world!");
}

//单元测试
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


