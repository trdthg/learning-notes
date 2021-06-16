
use std::fs;
use std::env;

fn main() {
    let args:Vec<String> = env::args().collect();
    let (query, filename) = parse_config(&args);
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    println!("With text:\n{}", contents);
}

fn parse_config(args:&[String]) -> (&str, &str) {
    let filepass = &args[0];
    let query = &args[1];
    let filename = &args[2];
    (query, filename)
}