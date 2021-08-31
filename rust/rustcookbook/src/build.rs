fn main() {
    cc::Build::new()
        .file("src/hello.c")
        .compile("hello");   // 输出 `libhello.a`
}
