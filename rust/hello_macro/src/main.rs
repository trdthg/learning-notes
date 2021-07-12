



use hello_macro::HelloMacro;

struct Pancakes;
impl HelloMacro for Pancakes {
    fn hello() {
        println!("Pancakes: Hello, Macro")
    }
}

fn main() {
    Pancakes::hello();
}
