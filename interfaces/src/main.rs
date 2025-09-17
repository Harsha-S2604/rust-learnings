use std::fmt;

struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

struct Tweet {
    pub username: String,
    pub content: String,
    pub replies: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn notify<T: Summary>(item: T) {
    println!("Breaking News! {}", item.summarize());
}

/* fn notify<T: Summary + fmt::Display>(item: &T) {
    println!("Breaking News! {}", item.summarize());
}*/

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

fn main() {
    let post = Tweet {
        username: String::from("John"),
        content: String::from("Tweet content"),
        replies: true,
        retweet: false,
    };

    notify(&post);
}
