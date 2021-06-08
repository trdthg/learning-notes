use ferris_says::say; // from the previous step
use std::io;
use std::io::{stdout, BufWriter};
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    guess_number_game();
    loop_and_fn();
    struct_test();
    tuple_sruct_test()
}

fn guess_number_game() {
    say_hello();
    println!("This is the start {}", '😻');
    let secret_number = rand::thread_rng().gen_range(1..6);

    loop {
        println!("Please input your number");
        let mut foo = String::new();

        io::stdin().read_line(&mut foo)
            .expect("Failed to read line");
        let foo: usize = match foo.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
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
    println!("{}", add(1,2));
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
    name: String
}

impl User {
    fn construct(id: u32) -> User {
        User {id, name: String::from("默认名称")}
    }

    fn id2(&self) -> u32 {
        self.id * 2
    }
}

fn build_user(name: String) -> User {
    User {
        id: 1,
        name     // 字段名与参数名相同可以简写
    }
}

fn struct_test() {
    let user1 = build_user(String::from("张三"));
    let user2 = User {
        name: String::from("李四"),
        ..user1   // ..指定了未显式设置的字段应与user1有相同的值
    };
    let user3 = User::construct(3);
    println!("{:?}, {}, {}, {}, {}", user1.name, user1.name, user2.id,user2.name, user1.id2());
    println!("{:?}", user1);
    println!("{:#?}", user2);
    println!("{:?}", user3);
}

fn tuple_sruct_test() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0,1,2);
    let origin = Point(3,4,5);
    println!("{} {}", black.1, origin.1)
}

