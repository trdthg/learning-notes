// 实现自定义错误类型
// 1.实现fmt(可以通过Debug)
// 2.实现Error的Train, 需要根据是否有子错误类型决定是否重写source()方法
// pub trait Error: Debug + Display {
//     // snip
//     fn source(&self) -> Option<&(dyn Error + 'static)> { None }
// }

fn natural() {
    use std::error::Error;
    use std::fmt::Display;

    struct AnError;
    impl Display for AnError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "子类型错误~") // 随便实现以下
        }
    }
    impl Error for AnError {} // 没有子类型, 不做处理

    struct CustomError {
        err: ChildError,
    }
    impl Display for CustomError {
        fn fmt(&self, f: &mut std::fmt::Formater<'_>) -> std::fmt::Result {
            write!("{}", self.source.unwrap())
        }
    }
    impl Error for CustomError {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            Some(self.err)
        }
    }
}
