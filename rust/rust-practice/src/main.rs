use ferris_says::say; // from the previous step
use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::{stdout, BufWriter};
use std::collections::HashMap;
use std::fs::File;
use io::ErrorKind;
use std::io::Read;

fn normal_guess_game() {
    let secret_number = rand::thread_rng().gen_range(1..6);
    let mut i:u32 = 0;
    let mut j:u32 = 0;
    loop {
        let guess = rand::thread_rng().gen_range(-10..10);
        i += 1;
        if guess < 1 || guess > 5 {
            j += 1;
            continue;
        }
        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("猜对了，共猜了{}次,范围不对有{}次", i, j);
                break;
            },
            _ => continue,
        }
    }
}

fn safer_guess_game() {
    pub struct Guess {
        value: i32,
    } impl Guess {
        pub fn new(value: i32) {
            if value < 1 || value > 5 {
                panic!("范围不对")
            } else {
                Guess {value};
            }
        }
        pub fn value(&self) {
            self.value;
        }
    }

}

use std::error::Error;
fn main() -> Result<(), Box<dyn Error>>{
    // guess_number_game();
    // println!("{}", fib(1, 1, 110));
    // loop_and_fn();
    // struct_test();
    // tuple_sruct_test();
    // enum_test_define();
    // enum_test_match();
    // vec_test();
    // string_test();
    // hashMap_test();
    
    // panic_test();
    safer_guess_game();
    let f = File::open("hello.txt")?;
    Ok(())



}

fn panic_test() {
    // panic!("主动崩溃了");

    // let a = vec![1,2,3];
    // a[99];

    // let f = File::open("hello.txt");
    // 简单的处理
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("可能没有该文件{:?}", error)
    //     }
    // };

    // 更丰富的错误类型处理
    // let f = match f {
    //     Ok(file) => file,
    //     // Err(error) => {
    //     //     panic!("可能没有该文件{:?}", error)
    //     // }
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(error) => panic!("创建文件失败：{}", error),
    //         },
    //         other_error => panic!("未知错误，可能没有权限{}", error),
    //     }
    // };

    // 去掉了大量的match表达式
    // let f = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("打开文件失败{}", error);
    //         })
    //     } else {
    //         panic!("是文件打开失败之外的其他错误");
    //     }
    // });

    // let f = File::open("hello.txt").unwrap();  // unwrap返回Ok或Err（直接调用panic！）
    // let f = File::open("hello.txt").expect("反正报错了"); // 与unwrap相比，不会使用原生panic信息，更容易找到错误原因

    // 传播错误
    // fn read_username_from_file() -> Result<String, io::Error> {
    //     let f = File::open("hello.txt");
    //     let mut f = match f {
    //         Ok(file) => file,
    //         Err(e) => return Err(e),
    //     };
    //     let mut s = String::new();
    //     match f.read_to_string(&mut s) {
    //         Ok(_) => Ok(s),
    //         Err(e) => Err(e),
    //     }
    // }
    // read_username_from_file().expect("msg: &str");

    // 传播错误的简写：？运算符
    // fn read_username_from_file() -> Result<String, io::Error> {
    //     let mut f = File::open("hello.txt")?;
    //     let mut s = String::new();
    //     f.read_to_string(&mut s)?;
    //     Ok(s)
    // }
    
    // ? 加链式法则进一步简化
    // fn read_username_from_file() -> Result<String,io::Error>{
    //     let mut s = String::new();
    //     let mut f = File::open("hello.txt")?.read_to_string(&mut s)?;
    //     Ok(s)
    // }

    // read_file_to_String的官方解法
    // use std::fs;
    // fn read_username_from_file() {
    //     let mut s = fs::read_to_string("hello.txt");
    // }
}

fn hashMap_test() {
    // common
    let mut map = HashMap::new();
    map.insert(String::from("Blue"), 10);
    
    let teams = vec!["blue".to_string(), String::from("green")];
    let scores = vec![5, 3];
    let map: HashMap<_, _> = teams.iter().zip(scores).collect();

    let a = String::from("sss");
    let b = 1;
    let mut map = HashMap::new();
    // map.insert(a, b); // 这里插入后，a和b的所有权就被map所有
    // println!("{}", a); // 不能打印
    map.insert(&a, &b);  // 把值的指针插入，a，b仍然有效，但是必须保证map也有效
    println!("{}", a);
    println!("{}", match map.get(&"sss".to_string()) {
        Some(i32) => 1,
        None => 2,
    });
    for (key, value) in &map {
        println!("{}: {}", key, value);
    }
    
    // 默认插入策略是覆盖
    // 没有时才插入
    map.entry(&"sss".to_string()).or_insert(&2);

    // 过滤插入
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split(" ") {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

}

fn string_test() {
    // push_str(str)
    let mut s = "Hello ".to_string();
    s.push_str("World!");
    
    // push, push_str(&str)
    let mut s2 = "Hello ".to_string();
    s2.push('-');
    s2.push('>');
    s2.push(' ');
    let s2_ = "World!";
    s2.push_str(s2_);
    println!("{} {}",s2, s2_);

    // +, format!
    let s1 = String::from("Hello ");
    let s2 = String::from("World!");
    let s3 = s1 + &s2;
    println!("{}", s2); // s1无法打印，+ 运算符时调用的函数签名
    /*
    官方解释类似于这样：fn add(self, s: &str) -> String {
    &s2（&String）被强转为了 &str，当add调用时，&s2被变成了&s2[..],
    add没有获得str的所有权，所以s2仍然有效 
    */

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // let s = s1 + "-" + &s2 + "-" + &s3; // 不是拷贝，需要获取s1的所有权
    let s = format!("{}-{}-{}", s1, s2, s3); // 返回一个新的字符串，不会获取所有权

    // String不支持索引
    /*
    String 是一个 Vec<u8> 的封装
    1. "Hola"  
        len->4*1 
        这里每一个字母的 UTF-8 编码都占用一个字节
    2. "Здравствуйте" 
        len->12*2  
        这里每个 Unicode 标量值需要两个字节存储。因此一个字符串字节值的索引并不总是对应一个有效的 Unicode 标量值
    3. “नमस्ते”
        len-> 6*3 他的u8 ->[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
    */

    // slice获取片段
    let hello = "Здравствуйте";
    println!("{}", &hello[0..4]); // 需要准确指定长度，这里是4对应两个字符
    // 遍历字符串
    for c in "नमस्ते".chars() {println!("{}", c)};
    for c in "नमस्ते".bytes() {println!("{}", c)};



}

fn vec_test() {
    // common
    let mut v1: Vec<i32> = Vec::new();
    let v2 = vec![1,2,3];
    v1.push(1);
    let a: i32 = v2[1];
    let b: &i32 = &v2[1]; 
    // let c = &v2[3]; // 若越界则程序直接停止
    match v2.get(3) {
        Some(b) => println!("{}", b),
        None => println!("越界"),
    }
    println!("{} {}", a, b);

    // 不能先取出Vec中的某个元素，之后再向Vec中push新元素，扩容会重新分配元素的内存

    // loop
    for i in v2 {
        println!("{}", i);
    }
    
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    // enum
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        String(String),
    }
    let row = vec![
        SpreadsheetCell::Int(12),
        SpreadsheetCell::Float(0.5),
        SpreadsheetCell::String(String::from("row中的第三个枚举类")),
    ];



}

fn guess_number_game() {
    say_hello();
    println!("This is the start {}", '😻');
    let secret_number = rand::thread_rng().gen_range(1..6);
    loop {
        println!("Please input your number");
        let foo = rand::thread_rng().gen_range(1..6);
        println!("{}", foo);
        // let mut foo = String::new();

        // io::stdin()
        //     .read_line(&mut foo)
        //     .expect("Failed to read line");
        // let foo: usize = match foo.trim().parse() {
        //     Ok(num) => num,
        //     Err(_) => continue,
        // };
        match foo.cmp(&secret_number) {
            Ordering::Less => println!("To Small"),
            Ordering::Greater => println!("To Big"),
            Ordering::Equal => {
                println!("YouWin");
                break;
            }
        }
    }
    // 元组
    let tuple: (u32, f32, i128) = (1, 1.0, 500);
    println!("{}", tuple.2);
    // let (x, y, z) = (1, 1.0, 500);
    // 数组
    // let array = ["Mondy", "Tuesday", "Wedneaday", "Thursday", "Friday", "Satuaday", "Sunday"];
    // let array: [i32; 5] = [1,2,3,4,5];
    // let array = [3;5];

    let x = {
        let y = 1;
        y + 1
    };
    println!("{}", x);
}

fn fib(a: u128, b: u128, n: u32) -> u128 {
    if n == 0 {
        a
    } else {
        fib(a + b, a, n - 1)
    }
}

fn say_hello() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}

fn number() -> i32 {
    5
}

fn loop_and_fn() {
    println!("Hello, world!");
    println!("{}", add(1, 2));
    let x = number();
    println!("{}", x == 5);

    let x = if x == 5 {
        add(x, 1);
        add(x, 1)
    } else {
        add(x, -1)
    };

    println!("{}", x);

    let mut x = 1;
    x = loop {
        x += 1;
        if x == 10 {
            break x * 2;
        }
    };
    println!("{}", x);

    for num in (1..5).rev() {
        print!("{} ", num);
    }
    let s = "Hello World!";
    // let s = String::from("Hello World");
    println!("{}", get_first_word(&s[..]));
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

#[derive(Debug)]
struct User {
    id: u32,
    name: String,
}

impl User {
    fn construct(id: u32) -> User {
        User {
            id,
            name: String::from("默认名称"),
        }
    }

    fn id2(&self) -> u32 {
        self.id * 2
    }
}

fn build_user(name: String) -> User {
    User {
        id: 1,
        name, // 字段名与参数名相同可以简写
    }
}

fn struct_test() {
    let user1 = build_user(String::from("张三"));
    let user2 = User {
        name: String::from("李四"),
        ..user1 // ..指定了未显式设置的字段应与user1有相同的值
    };
    let user3 = User::construct(3);
    println!(
        "{:?}, {}, {}, {}, {}",
        user1.name,
        user1.name,
        user2.id,
        user2.name,
        user1.id2()
    );
    println!("{:?}", user1);
    println!("{:#?}", user2);
    println!("{:?}", user3);
}

fn tuple_sruct_test() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0, 1, 2);
    let origin = Point(3, 4, 5);
    println!("{} {}", black.1, origin.1)
}

fn enum_test_define() {
    // 1.不指定具体类型
    #[derive(Debug)]
    enum MyIpAddrKind1 {
        V4,
        V6,
    }
    
    #[derive(Debug)]
    struct MyIpAddr1 {
        kind: MyIpAddrKind1,
        address: String,
    }

    // 2. 指定具体类型
    let ip1 = MyIpAddr1{
        kind: MyIpAddrKind1::V4, 
        address: String::from("127.0.0.1")
    };
    println!("{}", ip1.address);
    #[derive(Debug)]
    enum MyIpAddrKind2 {
        V4(u8,u8,u8,u8),
        V6(String),
    }
    let ip2 = MyIpAddrKind2::V4(127, 0, 0, 1);
    let ip3 = MyIpAddrKind2::V6(String::from("1231432048"));
    println!("{:?} {:?}", ip2, ip3);
    // 3. 对枚举的接口
    enum Message {
        Quit,
        Move{x: i32, y: i32},
        Write(String),
        ChangeColor(i32, i32, i32),
    } impl Message {
        fn call(&self, other: Message) {
            println!("{}", String::from("self"));
            String::from("self");
        }
    }
    let m = Message::Write(String::from("m:String"));
    let n = Message::Quit;
    m.call(n);

    // 4. Option枚举类的特点
    let num1 = Option::Some(2);
    let num2 = Some(2);
    // let num3: None; //无效，需要先指定类型才能把他设为空
    let num4: Option<i32> = None;

    let a: i32 = 5;
    // let c = a + num1; // Option<i32> 与 i32 不是同一类型
    
}

fn enum_test_match() {
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
    }
    #[derive(Debug)]
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) ->u8 {
        match coin {
            Coin::Penny => {
                println!("you are so lucky!");
                1
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}", state);
                25
            },
        }
    }
    let coin = Coin::Quarter(UsState::Alabama);
    println!("{}, {}", value_in_cents(Coin::Dime), value_in_cents(coin));

    // _ 通配符
    fn switch_case(a: u8) {
        match a {
            1 => println!("one"),
            2 => println!("two"),
            3 => println!("three"),
            _ => (),
        }
    }

    // if let Coin::Quarter(UsState::Alabama) = coin {
    //     println!("")
    // }


}