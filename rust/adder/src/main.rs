
use std::fs;
use std::env;

fn main() {
    let args:Vec<String> = env::args().collect();
    let config: Config = parse_config(&args);
    println!("In file {}", config.filename);
    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");
    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

fn parse_config1(args:&[String]) -> (&str, &str) {
    let filepass = &args[0];
    let query = &args[1];
    let filename = &args[2];
    (query, filename)
}

fn parse_config(args: &[String]) -> Config {
    let query = args[0].clone();
    let filename = args[1].clone();
    Config {
        query,
        filename
    }
    // main 中的 args 变量是参数值的所有者并只允许 parse_config 函数借用他们，
    // 不过会比储存字符串数据的引用消耗更多的时间和内存。不过拷贝数据使得代码显得更加直白因为无需管理引用的生命周期，所以在这种情况下牺牲一小部分性能来换取简洁性的取舍是值得的。
}