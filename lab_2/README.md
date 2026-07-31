# Rust Lab: Control Flow and Pattern Matching

## About the Project

This project is a Rust lab exercise that focuses on **control flow** and **pattern matching**, two important concepts used in everyday Rust programming.

The program demonstrates how to make decisions using `if` and `match` statements, repeat actions with loops, and work with optional values using `Option` and `if let`. These examples help build a better understanding of Rust's syntax and how programs execute different code paths.

## Learning Goals

By completing this lab, i am able to:

- understand that match must be exhaustive in rust.
- use if let and while let for concise option handling.
- complete the trangle area arm using the Heron's formula.
- explain why option<T> is safer than using null.
- get my FizzBuzz to cover allfour cases: Fizz, Buzz, FizzBuzz and number 

## Project Files

```
.
├── Cargo.toml
├── Cargo.lock
├── src
│   └── main.rs
        exercise_1.rs
        exercise_2.rs
        exercise_3.rs
        exercise_4.rs
└── README.md
```

## Requirements

Rust and Cargo are installed.

I checked by running

```bash
rustc --version
cargo --version
```

## How to Build

Compile the project with:

```bash
cargo build
```

## How to Run

Run the program using:

```bash
cargo run
```

## What the Program Covers

### Conditional Statements

The project uses `if/else`, statements to make decisions based on different conditions.

### Loops

Several looping techniques are demonstrated, including:

- `loop`
- `while`
- `for`

### Enums and Match Expressions

Rust's `match` statement makes it easy to compare values and execute the correct block of code.

### Working with `Option`

The program also shows how to safely handle values that may or may not exist.

### Using `if let`

When only one pattern needs to be checked, `if let` provides a cleaner alternative to `match`.


## What I Learned

This exercise helped me become more comfortable with Rust's control flow features. I learned when to use different types of loops, how `match` can make code easier to read, and how pattern matching improves safety when working with optional values. It also gave me more practice writing Rust programs that are both readable and reliable.

## Tools Used

- Rust
- Cargo
- Visual Studio Code (or any Rust-compatible editor)

## Possible Improvements

Some features that could be added in the future include:

- Pattern matching with custom enums
- Match guards
- Struct and tuple destructuring
- Error handling using `Result`
- More interactive examples with user input

## License

This project was created as part of a Rust programming laboratory exercise for educational purposes.