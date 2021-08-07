use oopgui::Draw;
use oopgui::{Screen, Button};
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}
fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ]
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("Ok"),
            })
        ]
    };
    screen.run();
    // 只有 对象安全（object safe）的 trait 才可以组成 trait 对象
    // - 返回值类型不为 Self
    // - 方法没有任何泛型类型参数
    println!("Hello, world!");

    use oopgui::Post;
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

    println!("---------------------------------------");
    // 第18章

    // 1. if let
    let favorite_color: Option<&str> = None;
    // let favorite_color: Option<&str> = Some("this is my favorite color");
    let is_tuesday = false;
    let age: Result<u8, _> = "34s".parse();
    let age: Result<u8, _> = "34".parse();
    if let Some(_) = favorite_color {
        println!("1: favorite");
        println!("{:?}", favorite_color);
    } else if is_tuesday {
        println!("2: tuesday");
    } else if let Ok(age_) = age {
        // 注意 if let 也可以像 match 分支那样引入覆盖变量：if let Ok(age) = age 引入了一个新的覆盖变量 age，它包含 Ok 成员中的值。这意味着 if age > 30 条件需要位于这个代码块内部；不能将两个条件组合为 if let Ok(age) = age && age > 30，因为我们希望与 30 进行比较的被覆盖的 age 直到大括号开始的新作用域才是有效的。
        if age_ > 30 {
            println!("3: age > 30");
        } else {
            println!("4: age <= 30");
        }
    } else {
        println!("5: else");
    }
    println!("{:?}", age);

    // 2. while let
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // 3. for
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{}: {}", index, value);
    }

    // 4. let
    let (a, b, c) = (1, 2, 3);
    let (a, _, _) = (1, 2, 3);
    let (a, ..) = (1, 2, 3);
    fn foo(&(x, y): &(i32, i32)) {
        println!("{}, {}", x, y);
    }
    foo(&(1, 2));

    // refutability 可反驳模式
    let a: Option<i32> = None;
    // let Some(x) = a;
    // 如果 a 的值是 None，其不会成功匹配模式 Some(x)，表明这个模式( Some(x) )是可反驳的。

    // 可反驳
    if let Some(x) = a {
        println!("{}", x);
    } else {
        println!("因为是不可反驳模式，所以不进if语句");
    }

    // 不可反驳，
    if let x = 5 {
        println!("{}", x);
    };

    // 匹配模式全列举
    // - 匹配字面值
    let x = 1;
    match x {
        1 => println!("one"),
        // 多个模式
        2 | 3 => println!("two or three"),
        // 通过 ..= 匹配值的范围 <==> 4 | 5 | 6
        4..=6 => println!("four to six"),
        _ => println!("anything"),
    }   
    match 'a' {
        // 范围只允许用于数字或 char 值，因为编译器会在编译时检查范围不为空。char 和 数字值是 Rust 仅有的可以判断范围是否为空的类型。
        'a'..='z' => println!("是小写字母"),
        _ => println!("anything"),
    }   
    // - 匹配命名变量
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        // 这里的y是match引入的覆盖变量, 与外部的y无关
        Some(y) => {
            println!("Matched, y = {:?}", y);
            println!("{:?}", x);
        },
        _ => println!("Default case, x = {:?}", x),
    }

    // 解构并分解值
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    // 简写,不过必须与键匹配
    let Point { x, y } = p; 
    println!("{}{}{}{}", x, y, a, b);
    let p = Point { x: 0, y: 7 };
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    
    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => println!("The Quit variant has no data to destructure."),
        Message::Move { x, y } => println!("Move in the x direction {} and in the y direction {}",x,y),
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change the color to red {}, green {}, and blue {}",r,g,b),
        Message => {
        }
        _ => {
        }
    }

    // _ 忽略
    let foo = |_: i32, y: i32| {
        println!("{}", y);
    };
    foo(3, 4);

    let mut setting_value = None;
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value);

    // _x 忽略未使用的变量
    let _z = 5;
    let y = 10;
    // println!("{}", z);
    println!("{}", _z);

    let s = Some(String::from("Hello!"));
    // 以下划线开头的未使用变量仍然会绑定值，它可能会获取值的所有权
    if let Some(_x) = s {
        println!("不是None");
    }
    // println!("{}", s);

    // 匹配守卫，能够引入外部变量作为匹配条件
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        // 这里的y是覆盖变量
        Some(y) => println!("Got any"),
        // 这里的y是外部变量
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    // 模式匹配优先级 
    let x = 4;
    let y = false;
    match x {
        // => (4 | 5 | 6) if y
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    // @绑定：匹配同时，创建变量
    enum Hello {
        World {id: i32},
    }
    let msg = Hello::World{id: 5};
    match msg {
        Hello::World { id: aaa @ 1..=5 } => {
            println!("创建了新的可以在分支内使用的变量： {}", aaa);
        }
        Hello::World{ id: 6..=10 } => {
            println!("只能捕获");
        }
        Hello::World{ id } => {
            println!("均匹配");
        }
    }

    println!("");
}
