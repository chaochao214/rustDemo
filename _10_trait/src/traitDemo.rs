
//trait 实现数据摘要
pub trait Summary {
    fn summarize(&self) -> String;
}



pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// 相当于接口与实现
impl Summary for NewsArticle{
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
