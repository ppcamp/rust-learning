#![allow(dead_code)]
// pub trait Summary {
//   fn summarize(&self) -> String;
// }

pub trait Summary {
  fn summarize(&self) -> String {
    String::from("(Read more...)")
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
}

// if we use the code bellow, it'll use the default's trait method
// impl Summary for NewsArticle {}

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
}

// We can use a trait to set a "pseudo" generic function
pub fn notify(item: &impl Summary) {
  println!("Breaking news! {}", item.summarize());
}

// If we wanna to control the types in the function bellow ...
// ... we must to use this approach, 'cause using impl, we couldn't ...
// ... have such control
// - Both have the same type:
// pub fn notify<T: Summary>(item1: &T, item2: &T)
// - They can have different types:
// pub fn notify(item1: &impl Summary, item2: &impl Summary) {

// Implementing for multiples traits
// pub fn notify(item: &(impl Summary + Display)) {
// pub fn notify<T: Summary + Display>(item: &T) {

// When we're handling with multiples traits we can use the where clause

// Some way: fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
// Another way: fn some_function<T, U>(t: &T, u: &U) -> i32
//   where T: Display + Clone,
//         U: Clone + Debug
// {

fn returns_summarizable() -> impl Summary {
  Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
  }
}

use std::fmt::Display;

struct Pair<T> {
  x: T,
  y: T,
}

impl<T> Pair<T> {
  fn new(x: T, y: T) -> Self {
    Self { x, y }
  }
}

impl<T: Display + PartialOrd> Pair<T> {
  fn cmp_display(&self) {
    if self.x >= self.y {
      println!("The largest member is x = {}", self.x);
    } else {
      println!("The largest member is y = {}", self.y);
    }
  }
}
