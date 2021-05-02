//use core::fmt::Display;

fn main() {
    let article1 = NewsArticle {
        id: 1,
        title: String::from("Lorem ipsum"),
    };

    let tw1 = Tweet {
        author: String::from("ahartlohner"),
        content: String::from("Hello world"),
    };

    let summary1 = article1.summarize();
    println!("{}", summary1);
    println!("{}", article1.summarize_2());

    let summary2 = tw1.summarize();
    println!("{}", summary2);
    println!("{}", tw1.summarize_2());

    notify(&article1);
    notify(&tw1);
}

pub trait Summary {
    fn summarize(&self) -> String;

    fn summarize_2(&self) -> String {
        String::from("(Read more)...")
    }
}

pub struct NewsArticle {
    id: i32,
    title: String,
}

pub struct Tweet {
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> std::string::String {
        format!("{} - {}", self.id, self.title)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> std::string::String {
        format!("Tweet from {}", self.author)
    }

    fn summarize_2(&self) -> std::string::String {
        self.summarize() + ": " + &self.content
    }
}

// fn notify<T: Summary + Display>(item: &T) {
//     println!("Breaking news! {}", item.summarize())
// }

fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize())
}

fn returns_summarizable(is_tweet: bool) -> impl Summary {
    Tweet {
        author: String::from("ahartlohner"),
        content: String::from("Hello world 2"),
    }
}
