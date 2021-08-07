fn main() {
    unsafe_test();
    extern_test();
    static_test();

    defaultT_and_add();
    RHS_test();
    call_same_name_methods();
    super_trait_test();
    overrideFmn();

    newtype_test();

    fn_pointer();

    macro_rules_test();
}
//-------------------PART V---------------------

fn macro_rules_test() {
    #[macro_export]
    macro_rules! myvec {
        // 首先，一对括号包含了整个模式。接下来是美元符号（ $ ），后跟一对括号，捕获了符合括号内模式的值以用于替换后的代码。
        // $() 内则是 $x:expr ，其匹配 Rust 的任意表达式，并将该表达式记作 $x。
        // $() 之后的逗号说明一个可有可无的逗号分隔符可以出现在 $() 所匹配的代码之后。
        // 紧随逗号之后的 * 说明该模式匹配零个或更多个 * 之前的任何模式。
        ( $( $x:expr ),* ) => {
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x);
                )*
                temp_vec
            }
        };
    }
    let a = myvec![1, 2, 3];
}


//-------------------PART IV--------------------
fn fn_pointer() {
    fn add_one(x: i32) -> i32 {
        x + 1
    }
    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }
    let answer = do_twice(add_one, 1);
    println!("{}", answer);

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers
        .iter()
        .map(|i| i.to_string()) // 传递闭包
        .collect();
    let list_of_strings: Vec<String> = list_of_numbers
        .iter()
        .map(ToString::to_string) // 传递函数
        .collect();

    enum Status {
        Value(u32),
        Stop,
    }
    let list_of_statuses: Vec<Status> = 
        (0u32..20)
        .map(Status::Value)
        .collect();

    // 返回闭包
    // fn return_closure() -> Fn(i32) -> i32 {
    //     |x| x + 1
    // }
    fn return_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }

    // 返回函数
    fn return_fn() -> fn(i32) -> i32 {
        |x| x + 1
    }
    println!("{}", return_fn()(1)) ;
    println!("{}", return_closure()(1)) ;
}


//-------------------PART III-------------------
fn newtype_test() {
    type Kilometers = i32;
    let x: i32 = 5;
    let y: Kilometers = 5;

    type Thunk = Box<dyn Fn() + Send + 'static>;
    let f: Thunk = Box::new(|| println!("hi!"));
    println!("{},{},{}", x, y, x + y);
    use std::io::Error;
    use std::fmt;
    // trait Write {
    //     fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    //     fn flush(&mut self) -> Result<(), Error>;
    //     fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
    //     fn flush_all(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
    // }
    type Result<T> = std::result::Result<T, Error>;
    trait Write {
        fn write(&mut self, buf: &[u8]) -> Result<usize>;
        fn flush(&mut self) -> Result<()>;
        fn write_all(&mut self, buf: &[u8]) -> Result<()>;
        fn flush_all(&mut self, fmt: fmt::Arguments) -> Result<()>;
    }
}


//-------------------PART II--------------------
fn defaultT_and_add() {
    use std::ops::Add;
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    impl Add for Point {
        type Output = Point;
        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }
    println!("{:?}", Point{x:1,y:2} + Point{x:2,y:1});
}

fn RHS_test() {
    use std::ops::Add;
    #[derive(Debug)]
    struct Millimeters(u32);
    #[derive(Debug)]
    struct Meters(u32);
    impl Add<Meters> for Millimeters {
        type Output = Millimeters;
        fn add(self, other: Meters) -> Millimeters {
            Millimeters(self.0 + other.0 * 1000)
        }
    }
    impl Add<Millimeters> for Meters {
        type Output = Meters;
        fn add(self, other: Millimeters) -> Meters {
            Meters(self.0 + other.0 / 1000)
        }
    }
    println!("{:?}", Millimeters(1) + Meters(1));
    println!("{:?}", Meters(1) + Millimeters(1000));
}

fn call_same_name_methods() {
    // 有参
    trait Polit {
        fn fly(&self);
    }
    trait Wizard {
        fn fly(&self);
    }
    struct Human;
    impl Polit for Human {
        fn fly(&self) {
            println!("I'm a Polit");
        }
    }
    impl Wizard for Human {
        fn fly(&self) {
            println!("I'm a Wizard");
        }
    }
    impl Human {
        fn fly(&self) {
            println!("I'm a Human");
        }
    }
    let person = Human;
    Polit::fly(&person);
    Wizard::fly(&person);
    Human::fly(&person);
    person.fly();

    // 无参
    trait Animal {
        fn baby_name() -> String;
    }
    struct Dog;
    impl Animal for Dog {
        fn baby_name() -> String { String::from("animal") }
    }
    impl Dog {fn baby_name() -> String { String::from("dog") }}
    println!("{}", Dog::baby_name());
    // println!("{}", Animal::baby_name());
    // 完全限定语法来指定我们希望调用的是Dog 的 Animal 实现
    println!("{}", <Dog as Animal>::baby_name());
}

fn super_trait_test() {
    use std::fmt;
    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }
    struct Point {
        x: i32,
        y: i32,
    }
    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}, {}", self.x, self.y)
        } 
    }
    impl OutlinePrint for Point {}
    let point = Point{x:1, y:2};
    point.outline_print();
    
}

fn overrideFmn() {
    use std::fmt;
    // impl fmt::Display for Vec<T> {}
    struct Wrapper(Vec<String>);
    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}

//-------------------PART I---------------------

fn unsafe_test() {
    println!("Hello, world!");
    // 解引用裸指针
    let mut num = 5;
    // 可以在安全代码中 创建 裸指针，只是不能在不安全块之外 解引用 裸指针
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    println!("{:?}, {:?}", r1, r2);
    // 创建指向任意内存地址的裸指针
    let address = 0x12345usize;
    let r = address as *const i32;
    println!("{:?}", r);

    // 只有在unsafe内才能解引用
    unsafe {
        println!("{:?}, {:?}", *r1, *r2);
        // println!("{:?}", *r); // 虽然能打印，但会直接报错 

        // 一个主要的应用场景便是调用 C 代码接口
        // 另一个场景是构建借用检查器无法理解的安全抽象。
    }

    unsafe fn dangerous() {};
    unsafe {
        // 必须在unsafe内调用unsafe function
        dangerous();
    }

    // 创建不安全代码的安全抽象
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, [1, 2, 3]);
    assert_eq!(b, [4, 5, 6]);

    fn my_split_at_mut(vec_slice: &mut[i32], mid: usize) -> (&mut[i32], &mut[i32]) {
        let len = vec_slice.len();
        assert!(mid < len);
        // (&mut slice[..mid], &mut slice[mid..]) // 编译器不知道这两部分不会重合
        use std::slice;
        // 使用 as_mut_ptr 方法访问 slice 的裸指针
        let ptr = vec_slice.as_mut_ptr();
        unsafe {
            // 不安全代码：slice::from_raw_parts_mut 函数获取一个裸指针和一个长度来创建一个 slice。
            (
                slice::from_raw_parts_mut(ptr, mid), 
                slice::from_raw_parts_mut(ptr.add(mid), len-mid+100)
            )
        }
    }
    let (a, b) = my_split_at_mut(r, 3);
    assert_eq!(a, [1, 2, 3]);
    // assert_eq!(b, [4, 5, 6]);
}

fn extern_test() {
    // 通过extern函数调用外部代码
    extern "C" {
        fn abs(input: i32) -> i32;
    }
    unsafe {
        println!("{}", abs(-3));
    }
    // 从其他语言调用rust函数
    #[no_mangle]
    pub extern fn call_from_c() {
        println!("其他语言可以用call_from_c调用该函数，不然函数名会被编译过程修改");
    }
}

// 定义静态变量
fn static_test() {
    static HELLO_WORLD: &str = "Hello World!";
    // 常量与静态变量的另一个区别在于静态变量可以是可变的。访问和修改可变静态变量都是 不安全 的。
    static mut COUNTER: u32 = 0;
    fn add(inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }
    unsafe {
        add(3);
        println!("{}", COUNTER);
    }
}

fn unsafe_trait_test() {
    unsafe trait Foo {
        // methods go here
    }
    
    unsafe impl Foo for i32 {
        // method implementations go here
    }
}