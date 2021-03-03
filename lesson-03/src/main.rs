fn main() {
  // Constants are like macros in C, i.e., are changed in compiler time
  const MAX_VALUE: u64 = 15;
  println!(
    "The constants MAX_VALUE ({}) it's like an MACRO in C",
    MAX_VALUE
  );

  // Imutable variables are like constants in C
  let x: i64 = 3;
  let x: i64 = x * 6;
  println!("X value is {} ", x);

  // An immutable variable can be "edited" in code, changing it's type
  // (creating a new one with this name at this point)
  let x: f64 = 15.0;
  println!("This value is {}", x);

  // You only can reassign an variable using let keyword
  // You can create an mutable element and didn't not change it, however, it'll be an warning

  // let mut spaces = "   ";
  // let mut spaces = spaces.len();
  // println!("Spaces: {}", spaces);

  // addition
  let sum = 5 + 10;

  // subtraction
  let difference = 95.5 - 4.3;

  // multiplication
  let product = 4 * 30;

  // division
  let quotient = 56.7 / 32.2;

  // remainder
  let remainder = 43 % 5;

  println!(
    "{}\t{}\t{}\t{}\t{}",
    sum, difference, product, quotient, remainder
  );

  let heart_eyed_cat: char = '😻';
  println!("The cat is {}", heart_eyed_cat);

  let tup: (i32, f64, u8) = (500, 6.4, 1);
  let (_, y, _) = tup;
  println!("The value of y is: {}", y);

  // An array using i32 with 5 elements
  let a: [i32; 5] = [1, 2, 3, 4, 5];
  // An array for this
  // let months = [
  //   "January",
  //   "February",
  //   "March",
  //   "April",
  //   "May",
  //   "June",
  //   "July",
  //   "August",
  //   "September",
  //   "October",
  //   "November",
  //   "December",
  // ];
  // An array initialized with 3
  // let a = [3; 5];
  for i in a.iter() {
    println!("The value is {}", i);
  }

  another_function(3, 6);
  println!("The value after execute is {}", x);

  let y = {
    let x = 3;
    // The line bellow is an expression (its like an return)
    x + 1
  };
  println!("The value of y is: {}", y);

  println!("Five {}", five());

  let x: i32 = plus_one(5);
  println!("The value of x is: {}", x);

  let number = 6;

  if number % 4 == 0 {
    println!("number is divisible by 4");
  } else if number % 3 == 0 {
    println!("number is divisible by 3");
  } else if number % 2 == 0 {
    println!("number is divisible by 2");
  } else {
    println!("number is not divisible by 4, 3, or 2");
  }

  // Ternary
  let condition = true;
  let number = if condition { 5 } else { 6 };
  println!("The value of number is: {}", number);

  // Loop with some output
  let mut counter = 0;
  let result = loop {
    counter += 1;

    if counter == 10 {
      // We are passing the counter*2 as the break output;
      break counter * 2;
    }
  };
  println!("The result is {}", result);

  // Loop with an breakpoint defined
  let mut number = 3;
  while number != 0 {
    println!("{}!", number);

    number -= 1;
  }
  println!("LIFTOFF!!!");

  // For over numbers
  for number in (1..4).rev() {
    println!("{}!", number);
  }
  println!("LIFTOFF!!!");
}

fn another_function(x: i32, y: i32) {
  println!("The value of x is: {}", x);
  println!("The value of y is: {}", y);
}

fn five() -> i32 {
  5
}

fn plus_one(x: i32) -> i32 {
  // You can use the "statments form" with return keyword
  // return x + 1;
  // Or you can just use the expression form (without the comma)
  x + 1
}
