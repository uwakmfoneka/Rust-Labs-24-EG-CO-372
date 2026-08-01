# Rust Lab: Structs, Traits, and Error Handling

## About the Project

This lab explores three important concepts in Rust: **structs**, **traits**, and **error handling**. The goal is to understand how Rust organizes data, shares behavior between different types, and handles errors safely without causing the program to crash.

Throughout the exercise, different examples are used to show how these features work together in real programs.

## Learning Objectives

After completing this lab, I was able to:

- Define  struct with an associated constructor function
- understand the differsnce between '&self', '&mut self' and 'self' iin method signature
- implement a trait for a struct and use trait objects(dyn traits)
- propagate errors with the ? operator and understand why from is required
- make my custom error error type implement both debug and display

## Project Structure

```
.
├── Cargo.toml
├── Cargo.lock
├── src
│   └── main.rs
        exercise_1.rs
        exercise_2.rs
        exercise_3.rs
└── README.md
```

## Requirements

Rust is installed.

Checked with:

```bash
rustc --version
cargo --version
```

## Building the Project

Compiled the project using Cargo.

```bash
cargo build
```

## Running the Program

Run the application with:

```bash
cargo run
```

## Topics Covered

### Structs

Structs are used to group related data together into a single type.

---

### Traits

Traits allow different types to share common behavior.

Traits make code more flexible and easier to reuse.

---

### Error Handling

Instead of crashing when something goes wrong, Rust encourages handling errors using the `Result` type.

## What I Learned

This lab helped me understand how Rust organizes code using structs and traits. I also learned how Rust handles errors differently from many other programming languages. Instead of relying on exceptions, Rust encourages handling possible failures using `Result`, making programs safer and more reliable.

Overall, the exercise showed how these features work together to create clean, reusable, and maintainable code.

## Technologies Used

- Rust
- Cargo
- Visual Studio Code

## Future Improvements

Some ideas for extending this project include:

- Reading input from the user
- Loading and saving data from files
- Creating multiple structs that implement the same trait
- Using custom error types instead of strings
- Exploring generic traits and trait bounds

## License

This project was created as part of a Rust programming laboratory exercise and is intended for learning purposes.