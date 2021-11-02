// 参数传递
// 基础类型一般是在栈上
// 大小可变类型一般在堆上

const PI: f64 = 3.1415926;
static V: Vec<u8> = Vec::new();

fn where_is_pi() {
    let r = 10f64;
    println!(
        "addr(r): {:p}, addr(PI): {:p}, addr(PI1): {:p}, area is: {}",
        &r,
        &PI,
        &V,
        PI * r * r
    );
}

fn main(){
    where_is_pi();
}