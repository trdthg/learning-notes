use ferris_says::say; // from the previous step
use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::{stdout, BufWriter};

fn main() {
    guess_number_game();
    println!("{}", fib(1, 1, 110));
    loop_and_fn();
    struct_test();
    tuple_sruct_test();
    enum_test_define();
    enum_test_match();
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
    fn switch_case(a: u8) ->u8 {
        match a {
            1 => println!("one"),
            2 => println!("two"),
            3 => println!("three"),
            _ => (),
        }
    }

    if let Coin::Quarter(Alabama) = coin {
        println!("")
    }


}