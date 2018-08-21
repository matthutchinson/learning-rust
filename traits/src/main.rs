// public Summary trait, having 3 method signatures, with summarize_again having a default
// implementation
pub trait Summary {
    fn summarize(&self) -> String;

    fn summarize_author(&self) -> String;

    fn summarize_again(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
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
        format!("{}: {}", self.username, self.content)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    fn summarize_again(&self) -> String {
        format!("(moar super fun tweets from {})", self.summarize_author())
    }
}

// here is a trait bound saying item must implement the Summary trait (since we call the summarize
// method)
pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {} - written by {}", item.summarize(), item.summarize_author());
}

fn main() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("Tweet summarized again! {}", tweet.summarize_again());
    println!("");

    println!("New article available! {}", article.summarize());
    println!("New article summarized again! {}", article.summarize_again()); // will be default impl
    println!("");

    notify(tweet);
    notify(article);
}
