// Brings the module context to this scope
mod some_mod;
mod utils;

// Import the test inside the module scope like
use some_mod::test;

// Use this library
use restaurant::eat_at_restaurant;

fn main() {
    test();
    utils::test();
    eat_at_restaurant();
    println!("Hello, world!");
}
