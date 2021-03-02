
<p align="center">
  <img
      src="https://doc.rust-lang.org/book/img/ferris/unsafe.svg"
      alt="Rust mascot"
      width="70px"
      height="70px"
  />
</p>

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
  - [6. Enums and Pattern Matching](#6-enums-and-pattern-matching)
  - [7. Managing Growing Projects with Packages, Crates, and Modules](#7-managing-growing-projects-with-packages-crates-and-modules)
  - [8. Common Collections](#8-common-collections)
  - [9. Error handling](#9-error-handling)
  - [10. Generic types, traits and lifetimes](#10-generic-types-traits-and-lifetimes)
  - [Usefull tips](#usefull-tips)
    - [Cargo:](#cargo)
    - [Adding functionalities to cargo](#adding-functionalities-to-cargo)


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

<br/>

## 5. Using structs to structure related data

[Reference][book-5]. In this section we learn how to create a rust structure (the formatting is quite similar to json objects in js). We also learn:
  - How define a struct object
  - How create a struct object using a tuple approach
  - An example using struct named and unnamed objects
  - The [debug][book-5-debug] flag. Used to print the objects as a json formatted
  - Putting a function as in [object oriented][book-5-poo] way method
  - How rust "autocompletes" the [function calling][book-5-dereferencing]

<br/>

## 6. Enums and Pattern Matching
[Reference][book-6]. In this chapter we see:
  - [How to handle][book-6-def] with `enum` type
  - The [Option<T> object][book-6-object], which contains the Some(val) and None
  - The [if let statment][book-6-iflet], which can be used to access the enum when you won't used all patterns defined
  - Acess [internal values in enums][book-6-match]
  - Enums used as an class type

<br/>

## 7. Managing Growing Projects with Packages, Crates, and Modules
[Reference][book-7]. In this chapter we learn how to handle with modules and libraries etc:
  - The entire module tree is rooted under the implicit module named crate.
  - An absolute path starts from a crate root by using a crate name or a literal crate
  - A relative path starts from the current module and uses self, super, or an identifier in the current module.
  - Rust put all items inside a module(functions, methods, structs, enums, modules, and constants) are private by default. However you can expose using keyword `pub`
  - You can use relative paths by putting the keyword `super` before the module
  - The `use` keyword bring the [module][book-7-use] to the current context
  - When using `enums` or `structs` it's recommended to use the full path-like, otherwise, you could use the `use` statments [to make them simplier][book-7-conventions].
  - You can use [aliases][book-7-alias] like js lang when imports.
  - You can re-export modules by putting a `pub` keyword before `use` statment
  - [External packages][book-7-external]
  - You can use nested paths, i.e, `use some::package::{self, some}`
  - You can glob, `use some::*`
  - The compiler will also look at `folder_module_name/mod.rs`, so this will work if I create a directory `example` containing a file `mod.rs`, see more by clicking [here][ext-1]
  - You can create a library with cargo using `cargo new --lib lib_name`, if so, you must add it in the same way you'll do when using crate packages, i.e, updating the `Cargo.toml` using a path to specify where to find them. Check it out by clicking [here][ext-2].
  - When you aren't using library approach you must push the module into scope with mod keyword;

> - Packages: A Cargo feature that lets you build, test, and share crates
> - Crates: A tree of modules that produces a library or executable
> - Modules and use: Let you control the organization, scope, and privacy of paths
> - Paths: A way of naming an item, such as a struct, function, or module

<br/>

## 8. Common Collections
[Reference][book-8]. In this section we see the "std" equivalent of `C++` for `Rust`. We cover in this section:
  - [Vectors][book-8-vec] and how to update them
  - The get method in vector, return us an `Option<T>` type
  - Ownership when concat strings
  - Due the [string memory management][book-8-str], you **can't** access string values as in the same way of `C++`, 'cause rust use 2Bytes for each utf-8 char, and index them using byte approach.
  - Like said in item above, you can't slice using int too, you'll need to use a byte repr way, if so.
  - [Hash][book-8-hash] will become the owner of the object.
  - Hash, in generall, overwrite the key.
  - Hash, by default, it's implemented [focusing in seccurity][book-8-hash-sc]

<br/>

## 9. Error handling
[Reference][book-9].
> Rust groups errors into two major categories: recoverable and unrecoverable errors.
Anothers infos:
  - When rust goes to an [unrecoverable error][book-9-error], it will `panic`, and so, it'll backtrace all path to clear memory, by default, however you can change this behavior.
  - Error [types][book-9-kinds]
  - `unwrap()` return the `Ok` value, if returned `Err`, will panic!
  - `expect()` same as `unwrap()`, however, let you to pass a new panic! message.
  - You can use the `Result` enum to [propagate][book-9-prop] errors (return it to main function or something like that)
  - [?][book-9-?] mark in rust is used to parse, if Ok, return it to code, if Err, return the err code to main. You only can use it in functions that return `Result<T,E>` type. However, it allows the main function too.


<br/>

## 10. Generic types, traits and lifetimes
[Reference][book-10]. In this section we see the "template" equivalent of `C++`.
<mark>This chapter explain one of the most important things about rust working</mark>
  - You can use a generic type to simplify functions.
  - There's a tradeoff in compilation time when using generic methods.
  - [Traits][book-10-traits] are like `interface` in other langs like *Java* or *Javascript*
  - To implement a trait, we must define a type and then, using `impl TRAITname for TYPEname`.
  - To use a trait for another module you must import them too
  - If we want to force the same type in generic parameters, we must use a [trait bound][book-10-traits-v] (the most verbose way)
  - We can use [multiples][book-10-multi-traits] implementations for traits
  - When handling with multiples traits, we can use the [where][book-10-where] clauses
  - We can return a trait instead of its type implementation
  - We can set a trait bound in a `impl`. In this approach, the generic trait will work like a [conditional][book-10-cond-trait]
  - Sometimes, you'll need to specify an lifetime anotation to rust. The [lifetimes][book-10-lifetime] annotations are usefull to prevent your code to go to `trash` values or `null`,
  - The rust can handle with functions that have one and-only-one parameter, that tooks a reference and return a reference in a function. This is because rust have some codes that are added into compiler. They are named as [lifetime elision rules][book-10-elision]. In another workds, `Rust` will infer the lifetime for references in basic cases.
  - The `static` lifetime will make the reference to be global in your code.

> The patterns programmed into Rust’s analysis of references are called the lifetime elision rules.



<br/>

---

## Usefull tips

### Cargo:
```bash
cargo build             # build the package
cargo check             # does the same process that build without building an executable
cargo clean             # clear the executables (target) directory
cargo init              # create a new rust package in the current directory
cargo new               # start a new rust package with a new dir
cargo new --lib newlib  # create new library named newlib
cargo r                 # execute the rust executable
cargo run               # ... the same as above
cargo search            # similar to pip search
cargo t                 # use the testing rust package to test the programs
cargo test              # ... the same as above
cargo update            # update the packages dependencies in Cargo.toml
```

### Adding functionalities to cargo
Tuto based on this [site](https://thenewstack.io/tutorial-import-libraries-of-rust-code-with-crates-io/)

```bash
cargo install cargo-edit  # install a rust binary that will give more power to cargo

# To install this tool you must to add the following packages libraries also
sudo apt install openssl libssl-dev

# This tool extends Cargo to allow you to add, remove, and upgrade dependencies by modifying your Cargo.toml file from the command line
cargo add package         # This automatically add a package into Cargo.toml file
cargo rm package          # This automatically remove a package from Cargo.toml file
cargo upgrade             # This automatically upgrade all your packages dependencies
```

<!-- Links --->

[ext-1]: https://stevedonovan.github.io/rust-gentle-intro/4-modules.html

[ext-2]: https://stackoverflow.com/questions/45519176/how-do-i-use-or-import-a-local-rust-file

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

[book-6]: https://doc.rust-lang.org/book/ch06-00-enums.html

[book-6-def]: https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html#defining-an-enum

[book-6-object]: https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html#the-option-enum-and-its-advantages-over-null-values

[book-6-match]: https://doc.rust-lang.org/book/ch06-02-match.html#the-match-control-flow-operator

[book-6-iflet]: https://doc.rust-lang.org/book/ch06-03-if-let.html

[book-7]: https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html

[book-7-use]: https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html

[book-7-conventions]: https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#creating-idiomatic-use-paths

[book-7-alias]: https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#providing-new-names-with-the-as-keyword

[book-7-external]: https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#using-external-packages

[book-8]: https://doc.rust-lang.org/book/ch08-00-common-collections.html

[book-8-vec]: https://doc.rust-lang.org/book/ch08-01-vectors.html

[book-8-str]: https://doc.rust-lang.org/book/ch08-02-strings.html

[book-8-hash]: https://doc.rust-lang.org/book/ch08-03-hash-maps.html#storing-keys-with-associated-values-in-hash-maps

[book-8-hash-sc]: https://doc.rust-lang.org/book/ch08-03-hash-maps.html#hashing-functions

[book-9]: https://doc.rust-lang.org/book/ch09-00-error-handling.html

[book-9-error]: https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html#unwinding-the-stack-or-aborting-in-response-to-a-panic

[book-9-kinds]: https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#matching-on-different-errors

[book-9-prop]: https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#propagating-errors

[book-9-?]: https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#propagating-errors

[book-10]: https://doc.rust-lang.org/book/ch10-00-generics.html

[book-10-traits]: https://doc.rust-lang.org/book/ch10-02-traits.html

[book-10-traits-v]: https://doc.rust-lang.org/book/ch10-02-traits.html

[book-10-multi-traits]: https://doc.rust-lang.org/book/ch10-02-traits.html#specifying-multiple-trait-bounds-with-the--syntax

[book-10-where]: https://doc.rust-lang.org/book/ch10-02-traits.html#clearer-trait-bounds-with-where-clauses

[book-10-cond-trait]: https://doc.rust-lang.org/book/ch10-02-traits.html#using-trait-bounds-to-conditionally-implement-methods

[book-10-lifetime]: https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-annotations-in-function-signatures

[book-10-elision]: https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-elision