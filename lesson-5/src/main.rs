#[allow(unused_imports)]
use std::io::stdin;

fn main() {
    // println!("Put the username: ");
    // let mut username = String::new();
    // stdin()
    //     .read_line(&mut username)
    //     .expect("Some error ocurred when parsing");

    // println!("Email: ");
    // let mut email = String::new();
    // stdin()
    //     .read_line(&mut email)
    //     .expect("Some error occurred when reading the line");

    // println!("Password: ");
    // let mut buff = String::new();
    // stdin()
    //     .read_line(&mut buff)
    //     .expect("Some error ocurred when reading the line");
    // let guess: i64 = match buff.trim().parse::<i64>() {
    //     Ok(num) => num,
    //     Err(err) => {
    //         println!("The cause of the error is {}", err);
    //         52
    //     }
    // };

    // let new_user = User {
    //     username,
    //     email,
    //     password: guess,
    //     active: true,
    // };
    // println!("User.email {}", new_user.email);

    // struct Point(i32, i32, i32);
    // let point = Point(3, 4, 5);

    // println!("Point ({},{},{})", point.0, point.1, point.2);

    let rectangle = Rectangle {
        width: 40,
        height: 50,
    };
    println!("{}", area(&rectangle));

    println!("{:?}", rectangle);
    println!("{:#?}", rectangle);
    println!("{:#?}", rectangle.get_area());
}

#[allow(dead_code)]
struct User {
    username: String,
    email: String,
    password: i64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn get_area(&self) -> String {
        let s = format!("The area is {}", self.height * self.width);
        String::from(s)
    }
    // Function used to compare
    #[allow(dead_code)]
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn area(rect: &Rectangle) -> u32 {
    rect.height * rect.width
}
