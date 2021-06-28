use oopgui::Draw;
use oopgui::{Screen, Button};
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}
fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ]
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("Ok"),
            })
        ]
    };
    screen.run();
    // 只有 对象安全（object safe）的 trait 才可以组成 trait 对象
    // - 返回值类型不为 Self
    // - 方法没有任何泛型类型参数
    println!("Hello, world!");

    use oopgui::Post;
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
