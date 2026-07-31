# Rust Lab: Ownership and Borrowing

## Overview

This lab exercise demonstrates Rust's core memory safety concepts: **ownership**, **borrowing**, and **references**. The project contains examples that illustrate how Rust manages memory without a garbage collector by enforcing ownership rules at compile time.

The exercises are intended for students learning the Rust programming language and provide practical examples of immutable borrowing, mutable borrowing, and ownership transfer.

## Objectives

By completing this lab, you will learn how to:

- Create and run a cargo project from scratch, Understand Rust's ownership model.
- Pass values by ownership and by reference.
- Use immutable references (`&T`) safely.
- Use mutable references (`&mut T`) while following Rust's borrowing rules.
- understand the difference between let and let mut, what move semantics meansand why 
  s1 is invalid 
- I completed all TODOS and my code compiles without warning

## Project Structure

.
├── Cargo.toml
├── Cargo.lock
├── src
│   └── main.rs
        exercise_1.rs
        exercise_2.rs
        exercise_3.rs
└── README.md

## Prerequisites

- Rust (latest stable version)
- Cargo (installed with Rust)

Verify installation:

```bash
rustc --version
cargo --version
```

## Building the Project

Clone the repository and navigate to the project directory.

```bash
cargo build
```

## Running the Program

Execute the application with:

```bash
cargo run
```

## Concepts Demonstrated

### Ownership

Each value in Rust has a single owner. When ownership is transferred, the previous owner can no longer access the value.

Example concepts:

- Variable moves
- Ownership transfer to functions
- Returning ownership

### Immutable Borrowing

Functions can borrow data without taking ownership.

### Borrow Checker

The Rust compiler prevents:

- Multiple mutable references
- Mutable and immutable references existing simultaneously
- Dangling references

These compile-time checks eliminate many common memory safety issues.

## Technologies Used

- Rust
- Cargo

## Future Improvements

Possible extensions include:

- Lifetimes
- Smart pointers (`Box`, `Rc`, `Arc`)
- Interior mutability (`RefCell`, `Cell`)
- Ownership in structs and enums
- Error handling with `Result` and `Option`

## License

This project is intended for educational purposes as part of a Rust programming lab.