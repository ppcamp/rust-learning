
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


<br/>
<br/>
<br/>

### 1. Getting started

In this section we learn how to install the `rust` env, of course, only the basic. You can find another tips
in the [cookbook][1] , in the [cargo book][2],

```bash
cargo check # not build, just check the project for erros
cargo build # create the exec
cargo run   # runs the project
```

### 2. Programming a Guessing Game
In this section whe learn:
   * How to add a new package:
   * handle with input (similar to Java, you'll need to parse every input made as a char buffer)
   * rand numbers
   * match introduction (arrow functions)
   * error handling
Check it out by clicking [here][3]


### 3. Common programming Concepts

In this section we see some of the rust's concepts:
  - The **let** keyword allows you to change an imutable variable, i.e., you'll create
    an variable and will destroy the variable used before.
  - An `constant` it's like an macro in C programming
  - The basics [types][4]
  - The [`char`][5] rust type
  - Tuples and its destructuring operation
  - Rust code uses `snake_case` as the conventional style for function and variable names.
  - Rust doesn’t care where you define your functions, only that they’re defined somewhere.
  - Expressions do not include ending semicolons.
  - [Conditionals][6] (if statments)
  - [Loops][7] and `range` type



<!-- Links --->
[1]: https://doc.rust-lang.org/book/ch01-00-getting-started.html
[2]: https://doc.rust-lang.org/cargo/getting-started/installation.html
[3]: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
[4]: https://doc.rust-lang.org/book/ch03-02-data-types.html#scalar-types
[5]: https://doc.rust-lang.org/book/ch03-02-data-types.html#the-character-type
[6]: https://doc.rust-lang.org/book/ch03-05-control-flow.html
[7]: https://doc.rust-lang.org/book/ch03-05-control-flow.html