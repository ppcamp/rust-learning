#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    // let ip_type = IpAddrKind::V4;
    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };
    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

    // // Which can be rewrite as
    // let home = IpAddr2::V4(String::from("127.0.0.1"));
    // println!("Home: {:#?}", home);

    // let some_number = Some(5);
    // let some_string = Some("a string");
    // let absent_number: Option<i32> = None;
    // println!("{:#?}", some_string);

    let coin = Coin::BRL(3.0);
    println!("The coinalue in usd {}", coin.to_usd());

    let v = plus_one(Some(5));
    let v = plus_one(None);

    let v = 1;
    match v {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("Other values"),
    }

    // print
    if let Coin::BRL(val) = coin {
        println!("R$ {}", val);
    } else if let Coin::USD(val) = coin {
        println!("$ {}", val);
    } else if let Coin::OTHER(val, _) = coin {
        println!("{}", val);
    }
}

/*
enum IpAddrKind {
    V4,
    V6,
}
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddr2 {
    // Could also use an enum type
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
// is quite similar to
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct
*/

enum Coin {
    BRL(f64),
    USD(f64),
    OTHER(f64, f64),
}
impl Coin {
    fn to_usd(&self) -> f64 {
        match self {
            Coin::BRL(val) => val * 5.89,
            Coin::USD(val) => val * 1.0,
            Coin::OTHER(val, relation) => val * relation,
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
