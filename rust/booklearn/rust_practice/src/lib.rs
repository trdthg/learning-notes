// 注意： trait 或者要实现 trait 的类型必须至少有一个位于 crate 的本地作用域
pub trait Summary {
    fn summarize(&self) -> String;
    fn summarize2(&self) -> String {
        String::from("默认实现")
    }
    fn summarize3(&self) -> String {
        format!("牛逼呀{}", self.summarize())
    }
}

pub trait Summary2 {
    
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}-{}-{}", self.headline,self.author,self.location)
    }
}
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}-{}", self.username, self.content)
    }
}