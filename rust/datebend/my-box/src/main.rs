use std::ops::Deref;

use my_box::MySmartPointer;

fn main() {
    println!("Hello, world!");

    let x = 5;
    let a = MySmartPointer::new(x);
    let b = Box::new(x);
    assert_eq!(x, *a.deref());
    assert_eq!(x, *a);
    assert_eq!(x, *b.deref());
    assert_eq!(x, *b);

    print!("{}", a.to_string());

    let s = String::from("aaaa");
    fn take_str(s: &str) {
        print!("{}", s);
    }
    take_str(&s);
}
