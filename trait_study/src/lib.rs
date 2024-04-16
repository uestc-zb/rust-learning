// fn main() {
//     let tweet = Tweet {
//         username: String::from("horse_ebooks"),
//         content: String::from("of course, as you probably already know, people"),
//         reply: false,
//         retweet: false,
//     };

//     println!("1 new tweet: {}", tweet.summarize());
// }
pub trait Summary {
    fn summarize_author(&self) ->String;
    
    fn summarize(&self) -> String{
        format!("(Read more...) {}", self.summarize_author())
    }
}
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for Tweet {
    fn summarize_author(&self) ->String {
        format!("@{}", self.username)
    }
    // fn summarize(&self) -> String{
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// impl Summary for Tweet {
//     fn summarize(&self) -> String {
//         format!("{}: {}", self.username, self.content)
//     }
// }
pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// fn largest<T: PartialOrd>(list: &[T]) -> T {}