
fn return_str() -> &str {
    let mut a = "aaa".to_string();
    for i in 1..3 {
        a.push_str("111");
    }
    return &a[..]
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn run() {
        println!(":#?")
    }
}
