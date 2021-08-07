
use std::fs;
use std::env;

fn main() {

    // 第12节
    // let args:Vec<String> = env::args().collect();
    // let (query, filename) = parse_config(&args);
    // println!("In file {}", filename);
    // let contents = fs::read_to_string(filename)
    // .expect("Something went wrong reading the file");
    // println!("With text:\n{}", contents);

    // let config: Config = parse_config2(&args);

    // let config: Config = Config::new(&args);
    
    // let config: Config = match Config::new(&args) {
    //     Ok(config) => config,
    //     Err(error) => panic!("{}",error),
    // };
    // let config: Config = Config::new(&args).expect("msg: &str");

    // 本地调用
    // let config: Config = Config::new(&args).unwrap_or_else(|error| {
    //     println!("Problem parsing arguments {}", error);
    //     std::process::exit(1);
    // });

    // if let Err(e) = run(config) {
    //     println!("Application error: {}", e);
    //     std::process::exit(1);
    // }

    // 库函数调用
    // eprintln! 配合 cargo run to poem.txt > output.txt
    // 如果正常结果会重定向到output，错误则不会
    // println！不管正确错误都会重定向
    // let config: adder::Config = adder::Config::new(&args).unwrap_or_else(|error| {
    //     println!("Problem parsing arguments {}", error);
    //     std::process::exit(1);
    // });

    // if let Err(e) = adder::run2(config) {
    //     println!("Application error: {}", e);
    //     std::process::exit(1);
    // }

    // 第13节
    // let simulated_user_specified_value = 10;
    // let simulated_random_number = 7;
    // generate_workout(simulated_user_specified_value, simulated_random_number);

    // // 闭包捕获环境
    // let x = 1;
    // let equal_to_x = |y| y == x;
    // // let equal_to_x2 = move |y| y == x;  
    // // 加了move后会强制获取变量的所有权
    // println!("{}", x);
    // println!("{}", equal_to_x(1));
    // println!("{}", x);
    // // 迭代器
    // let v1 = vec![1,2,3];
    // let mut v1_iter = v1.iter();
    // for i in v1_iter {
    //     println!("{}", i);
    // }
    // // for val in v1 {
    // //     println!("Got {}", val);
    // // }
    // // 使用 for 循环时无需使 v1_iter 可变因为 for 循环会获取 v1_iter 的所有权并在后台使 v1_iter 可变
    // let total: i32 = v1_iter.sum();
    // v1_iter = v1.iter();
    // assert_eq!(v1_iter.next(), Some(&1));
    // assert_eq!(v1_iter.next(), Some(&2));
    // assert_eq!(v1_iter.next(), Some(&3));
    // assert_eq!(v1_iter.next(), None);
    // println!("{}", total);

    // let mut v2: Vec<_> = v1.iter().map(|x| x+1).collect();
    // println!("{}", v2[0]);
    #[derive(Debug)]
    struct Counter {
        count: u32
    } impl Counter {
        fn new() -> Counter {
            Counter {count: 0}
        }
    } impl Iterator for Counter {
        type Item = u32;
        fn next(&mut self) -> Option<Self::Item> {
            self.count += 1;
            if self.count < 6 {
                Some(self.count)
            } else {
                None
            }
        }
    }

    fn using_other_iterator_trait_methods() {
        let sum: u32 = Counter::new().zip(Counter::new().skip(1))
                                    .map(|(a, b)| a * b)
                                    .filter(|x| x % 3 == 0)
                                    .sum();
        println!("{:?}", Counter::new());
        println!("{:?}", Counter::new().zip(Counter::new().skip(1)));
        println!("{:?}", Counter::new().zip(Counter::new().skip(1)).map(|(a, b)| a * b));
        println!("{:?}", Counter::new().zip(Counter::new().skip(1)).map(|(a, b)| a * b).filter(|x| x % 3 == 0));
        println!("{}", sum);
    }
    // using_other_iterator_trait_methods();

    // 第13节 通过Iterator重构第12节的config读取方式
    struct Config {
        query: String,
        filename: String,
    } impl Config {
        fn new(mut args: env::Args) -> Result<Config, &'static str> {
            args.next();
            let query = match args.next() {
                Some(arg) => arg,
                None => return Err("Didn't get a query string")
            };
            let filename = match args.next() {
                Some(arg) => arg,
                None => return Err("Didn't get a file name"),
            };
            Ok(Config { query, filename })
        }
    }

    let config:Config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });
}

use std::thread;
use std::time::Duration;
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("Calculating slowly..., intensity: {}", intensity);
    thread::sleep(Duration::from_secs(2));
    intensity
}
fn generate_workout(intensity: u32, random_number: u32) {
    // let expensive_result = simulated_expensive_calculation(intensity);
    // let expensive_closure = |num| {
        //     println!("calculating slowly...");
        //     thread::sleep(Duration::from_secs(2));
        //     num
        // };
    let mut cacher = Cacher::new(|num| {
        println!("Calculating slowly..., intensity: {}", num);
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        // println!("Today, do {} pushups", expensive_result);
        // println!("Next, do {} situps", expensive_result)
        // println!("Today, do {} pushups", expensive_closure(intensity));
        // println!("Next, do {} situps", expensive_closure(intensity));
        println!("Today, do {} pushups", cacher.value(intensity));
        println!("Next, do {} situps", cacher.value(intensity))
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            // println!("Today, run for {} minutes!", expensive_result);
            // println!("Today, run for {} minutes!", expensive_closure(intensity));
            println!("Today, run for {} minutes!", cacher.value(intensity));
        }
    }
}

struct Cacher<T>
    where T: Fn(u32) -> u32 {
    calculation: T,
    value: Option<u32>,
} impl<T> Cacher<T>
    where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {calculation, value: None,}
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}


// fn parse_config(args:&[String]) -> (&str, &str) {
//     let filepass = &args[0];
//     let query = &args[1];
//     let filename = &args[2];
//     (query, filename)
// }

// struct Config {
//     query: String,
//     filename: String,
// }

// impl Config {
//     // fn new(args: &[String]) -> Config {
//     //     // 错误处理
//     //     if args.len() < 3 {
//     //         panic!("not enough arguments!");
//     //     }
//     //     let query = args[0].clone();
//     //     let filename = args[1].clone();
//     //     Config {query, filename}
//     // }
//     fn new(args: &[String]) -> Result<Config, &'static str> {
//         if args.len() < 3 {
//             return Err("not enough arguments");
//         }
//         let query = args[1].clone();
//         let filename = args[2].clone();
//         Ok(Config { query, filename })
//     }
// }

// fn parse_config2(args: &[String]) -> Config {
//     let query = args[1].clone();
//     let filename = args[2].clone();
//     Config {query, filename}
// }

// fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
//     let contents = fs::read_to_string(config.filename)?;
//     println!("With text:\n{}", contents);
//     Ok(())
// }
