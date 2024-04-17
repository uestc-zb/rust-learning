use std::fmt::Display;
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("Hello, world! {}", result);

    let novel = String::from("Call me ...");
    let first_sentence = novel.split('.')
        .next()
        .expect("could not find a '.'");
    let i = ImportantExcerpt {part: first_sentence};
    println!("{}", i.part);

    let s: &'static str = "I have a static lifetime.";
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

fn longest_with_an_announcement<'a, T> (x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Annou {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}