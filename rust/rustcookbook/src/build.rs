fn main() {
    cc::Build::new()
        // 设置c文件
        .file("src/hello.c")
        // 设置宏
        // .define("APP_NAME", "\"foo\"")
        // .define("VERSION", format!("\"{}\"", env!("CARGO_PKG_VERSION")).as_str())
        // .define("WELCOME", None)
        // 设置输出文件
        .compile("hello"); // 输出 `libhello.a`

    // 编译c++ 和 c有两个区别

    // 一是通过构造器方法 cpp(true) 指定 C++ 编译器；
    // build.rs
    // cc::Build::new()
    //     .cpp(true)
    //     .file("src/hello.cpp")
    //     .compile("foo");

    // // 二是通过在 C++ 源文件顶部添加 extern "C" 代码段，以防止 C++ 编译器的名称篡改。
    // hello.cpp
    // extern "C" {
    //     int multiply(int x, int y);
    // }
    // int multiply(int x, int y) {
    //     return x*y;
    // }
}
