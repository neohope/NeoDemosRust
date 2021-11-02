// 字符串测试
// 胖指针，但具体字符串是一致的

fn main() {
    let data: String = "hello".into();

    let s1: &str = &data;
    let s2: &str = &data;
    let s3: &str = &data;

    dbg!(&s1 as *const _);
    dbg!(&s2 as *const _);
    dbg!(&s3 as *const _);

    dbg!(s1.as_bytes() as *const _);
    dbg!(s2.as_bytes() as *const _);
    dbg!(s3.as_bytes() as *const _);
}
