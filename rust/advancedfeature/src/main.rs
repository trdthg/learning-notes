fn main() {
    // unsafe_test();
    // extern_test();
    static_test();

}



fn unsafe_test() {
    println!("Hello, world!");
    // 解引用裸指针
    let mut num = 5;
    // 可以在安全代码中 创建 裸指针，只是不能在不安全块之外 解引用 裸指针
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    println!("{:?}, {:?}", r1, r2);
    // 创建指向任意内存地址的裸指针
    let address = 0x12345usize;
    let r = address as *const i32;
    println!("{:?}", r);

    // 只有在unsafe内才能解引用
    unsafe {
        println!("{:?}, {:?}", *r1, *r2);
        // println!("{:?}", *r); // 虽然能打印，但会直接报错 

        // 一个主要的应用场景便是调用 C 代码接口
        // 另一个场景是构建借用检查器无法理解的安全抽象。
    }

    unsafe fn dangerous() {};
    unsafe {
        // 必须在unsafe内调用unsafe function
        dangerous();
    }

    // 创建不安全代码的安全抽象
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, [1, 2, 3]);
    assert_eq!(b, [4, 5, 6]);

    fn my_split_at_mut(vec_slice: &mut[i32], mid: usize) -> (&mut[i32], &mut[i32]) {
        let len = vec_slice.len();
        assert!(mid < len);
        // (&mut slice[..mid], &mut slice[mid..]) // 编译器不知道这两部分不会重合
        use std::slice;
        // 使用 as_mut_ptr 方法访问 slice 的裸指针
        let ptr = vec_slice.as_mut_ptr();
        unsafe {
            // 不安全代码：slice::from_raw_parts_mut 函数获取一个裸指针和一个长度来创建一个 slice。
            (
                slice::from_raw_parts_mut(ptr, mid), 
                slice::from_raw_parts_mut(ptr.add(mid), len-mid+100)
            )
        }
    }
    let (a, b) = my_split_at_mut(r, 3);
    assert_eq!(a, [1, 2, 3]);
    // assert_eq!(b, [4, 5, 6]);
}

fn extern_test() {
    // 通过extern函数调用外部代码
    extern "C" {
        fn abs(input: i32) -> i32;
    }
    unsafe {
        println!("{}", abs(-3));
    }
    // 从其他语言调用rust函数
    #[no_mangle]
    pub extern fn call_from_c() {
        println!("其他语言可以用call_from_c调用该函数，不然函数名会被编译过程修改");
    }
}

// 定义静态变量
fn static_test() {
    static HELLO_WORLD: &str = "Hello World!";
    // 常量与静态变量的另一个区别在于静态变量可以是可变的。访问和修改可变静态变量都是 不安全 的。
    static mut COUNTER: u32 = 0;
    fn add(inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }
    unsafe {
        add(3);
        println!("{}", COUNTER);
    }
}

fn unsafe_trait_test() {
    unsafe trait Foo {
        // methods go here
    }
    
    unsafe impl Foo for i32 {
        // method implementations go here
    }
}