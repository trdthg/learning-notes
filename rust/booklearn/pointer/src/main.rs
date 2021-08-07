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
fn main() {

    // 第15节
    // 使用Box <T>指向堆上的数据
    use crate::List:: {Cons, Nil};
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

    println!("------引用循环与内存泄漏------");
    #[derive(Debug)]
    enum List4 {
        Cons4(i32, RefCell<Rc<List4>>),
        Nil4,
    }
    use List4::{ Cons4, Nil4};
    impl List4 {
        fn tail(&self) -> Option<&RefCell<Rc<List4>>> {
            match self {
                Cons4(_, item) => Some(item),
                Nil4 => None,
            }
        }
    }
    let a = Rc::new(Cons4(5, RefCell::new(Rc::new(Nil4))));
    println!("a-count: {}", Rc::strong_count(&a));
    println!("a-next: {:?}", a.tail());

    let b = Rc::new(Cons4(10, RefCell::new(Rc::clone(&a))));
    println!("a-count: {}", Rc::strong_count(&a));
    println!("b-count: {}", Rc::strong_count(&b));
    println!("b-next: {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }
    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));
    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());
    println!("------Weak reference 树实例------");
    // 强引用代表如何共享 Rc<T> 实例的所有权，但弱引用并不属于所有权关系。他们不会造成引用循环，因为任何弱引用的循环会在其相关的强引用计数为 0 时被打断。
    use std::rc::Weak;
    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>,
    }
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![])
    });
    let branch = Rc::new(Node {
        value: 5,
        // parent: RefCell::new(Weak::new(&leaf)),
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    println!("leaf's parent is {:?}", leaf.parent.borrow().upgrade());

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![])
    });
    println!("leaf strong count is {}", Rc::strong_count(&leaf));
    println!("leaf weak count is {}", Rc::weak_count(&leaf));
    {
        let branch2 = Rc::new(Node {
            value: 5,
            // parent: RefCell::new(Weak::new(&leaf)),
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        println!("-------------------------");
        println!("branch strong count is {}", Rc::strong_count(&branch2));
        println!("branch weak count is {}", Rc::weak_count(&branch2));
        println!("leaf strong count is {}", Rc::strong_count(&leaf));
        println!("leaf weak count is {}", Rc::weak_count(&leaf));
        println!("-------------------------");
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch2);
        println!("branch strong count is {}", Rc::strong_count(&branch2));
        println!("branch weak count is {}", Rc::weak_count(&branch2));
        println!("leaf strong count is {}", Rc::strong_count(&leaf));
        println!("leaf weak count is {}", Rc::weak_count(&leaf));
        println!("-------------------------");
    }
    // println!("branch strong count is {}", Rc::strong_count(&branch2));
    // println!("branch weak count is {}", Rc::weak_count(&branch2));
    println!("leaf strong count is {}", Rc::strong_count(&leaf));
    println!("leaf weak count is {}", Rc::weak_count(&leaf));
    println!("-------------------------");


}


