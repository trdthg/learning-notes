fn main() {
    println!("Hello, world!");

    use std::thread;
    use std::time::Duration;
    let handle = thread::spawn(|| {
        for i in 11..50 {
            println!("{}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 6..10 {
        println!("{}", i);
        thread::sleep(Duration::from_millis(2));
    }
    // 强制等待线程执行完毕
    handle.join().unwrap();

    // 可以在参数列表前使用 move 关键字强制闭包获取其使用的环境值的所有权。这个技巧在创建新线程将值的所有权从一个线程移动到另一个线程时最为实用。
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("{:?}", v);
    });
    // drop(v);
    // println!("{:?}", v);
    handle.join().unwrap();
}
