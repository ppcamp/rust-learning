fn main() {
    // The code bellow works like an "const char* " in C++
    let s = "hello";
    println!("The string is {}", s);
    // Due to the behavior of be an type implemented at stack, will not allows you to push

    // The code bellow works like an actual string in C++, i.e., will allocate in heap
    // let mut s = String::from("hello");
    // s.push('!');
    // println!("The string is {}", s);

    // let a: i64 = 3;
    // let b: i64 = 6;
    // let b: i64 = a;
    // println!("B {} A {}", b, a);

    // The code bellow won't work because rust has only one pointer to heap for time ...
    // ... it means that s1 do not exist anymore when you assign the s2 to this portion of memory
    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("{}, world!", s1);

    // You can use clone to deep copy these values

    let s = String::from("Test string");
    let (s, slen) = get_string_lenght(s);
    println!("Which gives us {} with {} size", s, slen);
    // However, we can reach the same result without create a new variable in test_function
    let (slen, s) = get_string_length_2(s);
    println!("Which gives us {} with {} size", s, slen);

    // Using the type reference (&) it will not change the ownership (memory scope)
    let len = calculate_length(&s);
    println!("The length of '{}' is {}.", s, len);

    let mut s = String::from("test");
    println!("String: {}", s);
    // Note that we aren't changed the ownership of s, instead, we've send one mutable reference to this object
    set_string_endln(&mut s);
    println!("String: {}", s);

    let pos = first_word(&s);
    let slice = String::from(&s[..pos]);
    s.clear();
    println!("Started at: {} -> {}", pos, slice);
    // first_word(&s)
}

fn get_string_lenght(s: String) -> (String, usize) {
    // The code bellow won't work because already changed the ownership
    // (s, s.len())
    let lenght: usize = s.len();
    (s, lenght)
}

//  You can have only one function at a time (let behavior don't work here)
fn get_string_length_2(s: String) -> (usize, String) {
    (s.len(), s)
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn set_string_endln(s: &mut String) {
    s.push_str(". At the end");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
