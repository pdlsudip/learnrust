mod generics;
mod lifetimes;
mod traits;
use generics::genericfn;
use traits::Summary;
use traits::{NewsArticle, Tweet};
fn main() {
    genericfn();

    let article = NewsArticle {
        content: String::from("new News"),
        author: String::from("newN"),
        headline: String::from("Latest news"),
    };

    let tweet = Tweet {
        content: String::from("New tweet"),
        reply: true,
        retweet: true,
        username: String::from("newT"),
    };
    println!("Summary for tweet: {}", tweet.summarize());
    println!("Summary for article: {}", article.summarize());
}
