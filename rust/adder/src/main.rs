
use std::fs;
use std::env;

fn main() {
    let args:Vec<String> = env::args().collect();
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
    let config: adder::Config = adder::Config::new(&args).unwrap_or_else(|error| {
        println!("Problem parsing arguments {}", error);
        std::process::exit(1);
    });

    if let Err(e) = adder::run2(config) {
        println!("Application error: {}", e);
        std::process::exit(1);
    }

}

fn parse_config(args:&[String]) -> (&str, &str) {
    let filepass = &args[0];
    let query = &args[1];
    let filename = &args[2];
    (query, filename)
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    // fn new(args: &[String]) -> Config {
    //     // 错误处理
    //     if args.len() < 3 {
    //         panic!("not enough arguments!");
    //     }
    //     let query = args[0].clone();
    //     let filename = args[1].clone();
    //     Config {query, filename}
    // }
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

fn parse_config2(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();
    Config {query, filename}
}

fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(config.filename)?;
    println!("With text:\n{}", contents);
    Ok(())
}