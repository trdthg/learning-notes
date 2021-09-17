fn use_errorchain() {
    use error_chain::error_chain;
    error_chain! {
        // 把std Error映射为自定义Error
        foreign_links {
          Io(std::io::Error);
          StripPrefixError(::std::path::StripPrefixError);
        }
        // 自定义错误类型
        errors {
            Single {
                description("MyError!")
                display("Single Error")
            }
            Duple(t: String) {
                description("MyError!")
                display("Dutple {} Error", t)
            }
            Multi(len: u32, data: Vec<u32>) {
                description("MyError!")
                display("Multi len {} data {:?} Error", len, data)
            }
        }
    }
}
