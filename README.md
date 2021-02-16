
<img
    src="https://doc.rust-lang.org/book/img/ferris/unsafe.svg"
    alt="Rust mascot"
    width="50px"
    height="50px"
    style="display:block; margin:auto; width:50px; height:50px"
/>

# rust-learning
A repository that will be used to put everything that I've did when
I was reading the rust documentation and examples.

<div style="clear:both"></div>


- [rust-learning](#rust-learning)
  - [1. Getting started](#1-getting-started)
  - [2. Programming a Guessing Game](#2-programming-a-guessing-game)
  - [3. Common programming Concepts](#3-common-programming-concepts)
  - [4. Understanding Ownership](#4-understanding-ownership)
  - [5. Using structs to structure related data](#5-using-structs-to-structure-related-data)
  - [Usefull tips](#usefull-tips)


<br/>
<br/>
<br/>

## 1. Getting started

In this section we learn how to install the `rust` env, of course, only the basic. You can find another tips
in the [cookbook][book-1] , in the [cargo book][cargo-1],

```bash
cargo check # not build, just check the project for erros
cargo build # create the exec
cargo run   # runs the project
```

<br/>

## 2. Programming a Guessing Game
[Reference][book-2]. In this section whe learn:
   * How to add a new package:
   * handle with input (similar to Java, you'll need to parse every input made as a char buffer)
   * rand numbers
   * match introduction (arrow functions)
   * error handling

<br/>

## 3. Common programming Concepts

[Reference][book-3]. In this section we see some of the rust's concepts:
  - The **let** keyword allows you to change an imutable variable, i.e., you'll create
    an variable and will destroy the variable used before.
  - An `constant` it's like an macro in C programming
  - The basics [types][book-3-dtypes]
  - The [char][book-3-dtypes-char] rust type
  - Tuples and its destructuring operation
  - Rust code uses `snake_case` as the conventional style for function and variable names.
  - Rust doesn’t care where you define your functions, only that they’re defined somewhere.
  - Expressions do not include ending semicolons.
  - [Conditionals][book-3-cf] (if statments)
  - [Loops][book-3-loops] and `range` type


<br/>

## 4. Understanding Ownership

[Reference][book-4-ownership]. In this chapter we learn how rust handles with memory allocation and "de-allocation". Furthermore
we also learn:
  - The equivalent `const char*` in rust
  - How rust handles with multiples pointers to the same object
  - How rust [deallocate][book-4-memo] the memory for variables that uses Heap
  - When you pass some variable allocated in some scope to other function, it will be deleted in "main" scope
  - You can return an tuple
  - Rust only allows one mutable reference to a specific scope (`{}`), however, you can have multiples immutable references to that scope. This behavior prevents race conditions to this data at a compile time.
  - The rust blocks the creation of dangling references, it means that you can't create a `reference pointer` to some data in a particular scope that will be freed due to the ownership behavior.
  - The rust compiler seems to be based on variables scopes, i.e., when you try to do something like this:
    ```rust
    // Will raise an error at compile time
    let pos = first_word(&s);
    let slice = &s[..pos];
    s.clear();
    println!("Started at: {} -> {}", pos, slice);

    // Won't raise an error at compile time (will "panick")
    let pos = first_word(&s);
    s.clear();
    println!("Started at: {} -> {}", pos, &s[..pos]);
    ```
  - I think that we can't convert the "sliced" string to a real one due to the pointer used to, however, we can create another object and use reference of slice type to do so. The slice type are described [here][book-4-slice]


## 5. Using structs to structure related data

[Reference][book-5]. In this section we learn how to create a rust structure (the formatting is quite similar to json objects in js). We also learn:
  - How define a struct object
  - How create a struct object using a tuple approach
  - An example using struct named and unnamed objects
  - The [debug][book-5-debug] flag. Used to print the objects as a json formatted
  - Putting a function as in [object oriented][book-5-poo] way method
  - How rust "autocompletes" the [function calling][book-5-dereferencing]

---

## Usefull tips

Cargo:
```bash
cargo build  # build the package
cargo check  # does the same process that build without building an executable
cargo clean  # clear the executables (target) directory
cargo init   # create a new rust package in the current directory
cargo new    # start a new rust package with a new dir
cargo r      # execute the rust executable
cargo run    # ... the same as above
cargo search # similar to pip search
cargo t      # use the testing rust package to test the programs
cargo test   # ... the same as above
cargo update # update the packages dependencies in Cargo.toml
```



<!-- Links --->
[cargo-1]: https://doc.rust-lang.org/cargo/getting-started/installation.html

[book-1]: https://doc.rust-lang.org/book/ch01-00-getting-started.html

[book-2]: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

[book-3]: https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html

[book-3-dtypes]: https://doc.rust-lang.org/book/ch03-02-data-types.html#scalar-types

[book-3-dtypes-char]: https://doc.rust-lang.org/book/ch03-02-data-types.html#the-character-type

[book-3-cf]: https://doc.rust-lang.org/book/ch03-05-control-flow.html

[book-3-loops]: https://doc.rust-lang.org/book/ch03-05-control-flow.html#repeating-code-with-loop

[book-4-ownership]: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ownership-and-functions

[book-4-memo]: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#memory-and-allocation

[book-4-slice]: https://doc.rust-lang.org/book/ch04-03-slices.html#the-slice-type

[book-5]: https://doc.rust-lang.org/book/ch05-00-structs.html

[book-5-debug]: https://doc.rust-lang.org/book/ch05-02-example-structs.html#adding-useful-functionality-with-derived-traits

[book-5-poo]: https://doc.rust-lang.org/book/ch05-03-method-syntax.html#defining-methods

[book-5-dereferencing]: https://doc.rust-lang.org/book/ch05-03-method-syntax.html#wheres-the---operator