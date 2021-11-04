#[warn(unused_macros)]
macro_rules! syscall {
    ($fn: ident( $($args: expr), * $(,) *)) => {{
        let res = unsafe { libc:: }
    }};
}

fn main() {
    println!("Hello, world!");
}
