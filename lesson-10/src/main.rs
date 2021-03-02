// Include the traits module
mod traits;

// Import them ...
// ... @note that the trait must be imported too
use crate::traits::{notify, NewsArticle, Summary, Tweet};

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let str_list = vec!["d", "z", "f", "a", "c"];
    println!(
        "The largest number is: {}\n The larger char is: {}",
        largest(&number_list),
        largest(&str_list)
    );

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    let na = NewsArticle {
        headline: String::from("Test"),
        location: String::from("Pindamonhangaba"),
        author: String::from("coxinha"),
        content: String::from("None"),
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("News article: {}", na.summarize());

    notify(&tweet);
    notify(&na);

    let a = String::from("Testeeeeeeeeeeeeeeee");
    let b = String::from("De alguma coisa");
    println!("{}", longest(&a, &b));

    // explanation about how we must use lifetime specifier ...
    // ... without them, rust't can't tell where this code is valid ...
    // ... basing on its scope, i.e, {}
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
    // using the print bellow, it'll throw an error due to the lifetime ...
    // ... Note that in this function we may use the string2 as result, ...
    // ... however, the string2 goes out of scope, so, when we reference ...
    // ... to the string2 and then, we go out of sope, we pass to reference to `null` or ...
    // ... `trash`. To avoid this, rust force you to use lifetime annotation in this case. ...
    // ... Code:
    // println!("The longest string is {}", result);
}

// To fix this function, we must include another trait
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    // if we implement the copy we can return the ownership instead a reference
    // let mut largest = &list[0];
    let mut largest = list[0];

    // for item in list {
    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// The 'a is a lifetime anotation ...
// ... In practice, it means that the lifetime of the reference ...
// ... returned by the longest function is the same as the smaller ...
// ... of the lifetimes of the references passed in.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// struct Point<T> {
//     x: T,
//     y: T,
// }

// struct Point<T, U> {
//     x: T,
//     y: U,
// }
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }
// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

// Using traits parameters, bounds and lifetimes
use std::fmt::Display;

#[allow(dead_code)]
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
