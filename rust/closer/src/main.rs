fn main() {



}

fn closer_move() {
    // Fn只是借用
    let s = "sss".to_string();
    let f = || println!("{}", s);

    fn consume<T>(f: T)
    where T: Fn() {
        f();
    }

    consume(f);
}

fn closer_borrow_mut() {
    // FnMut 可变借用
    let mut s = "sss".to_string();
    let f = || s.push('c');

    fn consume<T>(mut f: T)
    where T: FnMut() {
        f();
    }

    consume(f);
}

fn closer_borrow() {
    // FnOnce交出所有权, 结束闭包后变量被析构
    let mut s = "sss".to_string();
    let f = || s.push('c');

    fn consume<T>(f: T)
    where T: FnOnce() {
        f();
    }

    consume(f);
}


fn return_closer() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

fn return_closer2() -> impl FnOnce() -> String {
    // s在堆中分配, 在结束闭包时通过FnOnce + move交出了所有权, 可以在外部使用
    let s = String::from("sss");
    move || s
}