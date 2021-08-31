// 二是通过在 C++ 源文件顶部添加 extern "C" 代码段，以防止 C++ 编译器的名称篡改。

extern "C" {
    int multiply(int x, int y);
}

int multiply(int x, int y) {
    return x*y;
}