use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Foo {
    bar: &'static str,
}

impl Drop for Foo {
    fn drop(&mut self) {
        println!("foo dropped!");
    }
}

fn main() {
    let foo = Rc::new(Foo { bar: "hello" });
    let weak_foo = Rc::downgrade(&foo);
    let other_weak_foo = Weak::clone(&weak_foo);
    
    drop(weak_foo); // 不会 drop
    drop(foo); // 打印 dropped
    
    // 此时还有 weak ref存在，但内容其实已经没有了
    println!("other weak foo: {:?}", other_weak_foo.upgrade());
    
    // 更不能去 upgrade
    assert!(other_weak_foo.upgrade().is_none());
    println!("other weak foo.upgrade().is_none()");
}
