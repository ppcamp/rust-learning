#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
use std::collections::HashMap;

fn main() {
    // u can use this approach
    let mut v: Vec<i64> = Vec::new();
    v.push(3);
    print!("{:#?}\n{}\n", v, "-".repeat(100));

    // u can also define as
    let vv = vec![1, 2, 3]; // which infer this type

    let mut third: &i32 = &vv[2]; // at pos 2
    let t: i32 = 3;
    third = &t;
    println!("{:#?}", vv);

    // matching with element
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // Iterate over itens (auto)
    for i in &vv {
        println!("{}", i);
    }

    // Updating elements
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("{:#?}", v);

    // storing utf-8
    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();

    let hello = String::from("こんにちは");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("{:?}", s3);
    // You can't do &s1+&s2, in this case, you should use another approach

    // iterating over the string chars
    for c in hello.chars() {
        println!("{}", c);
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("Score {}", scores["Blue"]);
    println!("Score {:?}", scores.get("Blue"));

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("{:#?}", scores);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    for (key, value) in &scores {
        println!("Scores[{}] = {}", key, value);
    }
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    // Check if key exist, if don't, create it
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    // Entry can be used to count the occurrences
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
