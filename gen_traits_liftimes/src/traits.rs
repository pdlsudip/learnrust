pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("Read more.... by @{}", self.summarize_author())
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("{}", self.username)
    }
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
    fn summarize(&self) -> String {
        format!("{} by @{}", self.headline, self.author)
    }
}
