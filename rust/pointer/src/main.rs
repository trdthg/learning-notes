
fn main() {
    // 第15节
    // 使用Box <T>指向堆上的数据
    use crate::List:: {Cons, Nil};
    println!("Hello, world!");
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // 自定义智能指针
    use std::ops::Deref;
    struct MyBox<T> (T);
    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    } impl<T> Deref for MyBox<T> {
        // 解引用强制多态
        type Target = T;
        fn deref(&self) -> &T {
            &self.0
        }
    }
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    fn hello(name: &str) {
        println!("Hello, {}!", name);
    }
    let m = &&&*&*&&&&MyBox::new(String::from("Rust"));
    hello(&m);

    // 使用Drop Trait在离开作用于后自动运行清理代码
    impl<T> Drop for MyBox<T> {
        fn drop(&mut self) {
            println!("sss");
        }
    }
    // 使用std::mem::drop主动显式清理
    std::mem::drop(y);
    println!("主动清理了m");

    // Rc引用计数
    let a = Cons(5,
        Box::new(Cons(10,
            Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    // println!("{:?}", a); // a已经被b所有
    use crate::List2::{Cons2, Nil2};
    let a = Rc::new(Cons2(
        5,  Rc::new(Cons2(
        10, Rc::new(Nil2)))));
    let b = Cons2(3, Rc::clone(&a));
    let c = Cons2(4, Rc::clone(&a));

    // 离谱的多个可变引用
    use std::cell::RefCell;
    use std::rc::Rc;
    use List3::{Cons3, Nil3};
    #[derive(Debug)]
    enum List3{
        Cons3(Rc<RefCell<i32>>, Rc<List3>),
        Nil3,
    }
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons3(
        Rc::clone(&value), Rc::new(Nil3)
    ));
    let b = Rc::new(Cons3(
        Rc::new(RefCell::new(4)), Rc::clone(&a)
    ));
    let c = Rc::new(Cons3(
        Rc::new(RefCell::new(3)), Rc::clone(&b)
    ));
    *value.borrow_mut() += 10;
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

}

// 递归类型
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}
use std::rc::Rc;
#[derive(Debug)]
enum List2 {
    Cons2(i32, Rc<List2>),
    Nil2,
} impl Drop for List2 {
    fn drop(&mut self) {
        println!("List2开始清理 {:?}", self);
    }
}
