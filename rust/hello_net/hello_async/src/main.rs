// #![feature(generators, generator_trait)]

fn main() {
    println!("Hello, world!");

}


pub fn gen() {
    use std::ops::Generator;
    let mut gen = || {
        println!("yield_1");
        yield 1;
        println!("2");
        yield 2;
        println!("3");
        yield 3;
        println!("4");
        yield 4;
    };

    use std::pin::Pin;
    let c = Pin::new(&mut gen).resume(());
    println!("consume_{:?}", c);
    let c = Pin::new(&mut gen).resume(());
    println!("consume_{:?}", c);
    let c = Pin::new(&mut gen).resume(());
    println!("consume_{:?}", c);
    let c = Pin::new(&mut gen).resume(());
    println!("consume_{:?}", c);
}