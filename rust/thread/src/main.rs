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

    println!("----------消息传递----------");
    // 使用消息传递在线程间传送数据
    use std::sync::mpsc;
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("Hi");
        tx.send(val).unwrap();
    });
    // loop {  // 不会用，看不懂
    //     let remsg = rx.try_recv().unwrap();
    //     println!("remsg: {}", remsg);
    // }
    let received = rx.recv().unwrap();
    println!("{}", received);

    println!("----------消息传递2----------");
    let (tx, rx) = mpsc::channel();
    let tx_clone = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    
    
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx_clone.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    let mut v1 = rx.iter();
    let a = v1.next();
    println!("{:?}", a);
    for received in rx {
        println!("Got: {}", received);
    }


    // 共享状态并发互斥器
    use std::sync::{Mutex, Arc};
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("count: {}", *counter.lock().unwrap());

    // 死锁尝试
    let a = Arc::new(Mutex::new("left".to_string()));
    let b = Arc::new(Mutex::new("right".to_string()));
    let a_ = Arc::clone(&a);
    let b_ = Arc::clone(&b);
    let handle_a = thread::spawn(move || {
        let mut got_a = a.lock().unwrap();
        println!("获取a正在等待b");
        thread::sleep(Duration::from_secs(1));
        let mut got_b = b.lock().unwrap();
        // let mut got_b = b.try_lock().unwrap();
    });
    let handle_b = thread::spawn(move || {
        let mut got_a = b_.lock().unwrap();
        println!("获取b正在等待a");
        thread::sleep(Duration::from_secs(1));
        let mut got_b = a_.lock().unwrap();
    });
    handle_a.join().unwrap();
    handle_b.join().unwrap();
}
