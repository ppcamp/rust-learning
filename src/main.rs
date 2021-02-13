use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io;

fn main() {
    // Get random numbers
    let mut rng = thread_rng();
    let secret: i64 = rng.gen_range(0..100);
    println!("Guess the number: {}", secret);

    // Infinite loop
    loop {
        println!("Put some value here: ");
        let mut buf = String::new();
        io::stdin()
            .read_line(&mut buf)
            .expect("Some error ocurred when reading the line");

        // Parse to integer 64
        let guess: i64 = match buf.trim().parse::<i64>() {
            Ok(num) => num,
            Err(err) => {
                println!("The cause of the error is {}", err);
                continue;
            }
        };

        match guess.cmp(&secret) {
            // The code bellow is named "arms" (similar to arrow functions in js) only works with `match`
            Ordering::Equal => {
                println!("You win");
                break;
            }
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
        }
    }
}
