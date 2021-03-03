#[allow(dead_code)]
mod restaurant {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Add to waitlist");
        }
        fn seat_at_table() {
            println!("Add to waitlist");
        }
    }

    mod serving {
        fn take_order() {
            println!("Take order");
        }
        fn take_payment() {
            println!("Take payment");
        }
        fn serve_order() {
            println!("Serve");
        }
    }
}

pub fn eat_at_restaurant() {
    println!("{}", "-".repeat(80));
    println!("Using library approach");

    // Absolute path
    crate::restaurant::hosting::add_to_waitlist();

    // Relative path
    restaurant::hosting::add_to_waitlist();

    println!("{}", "-".repeat(80));
}
