# Rust Lab: Lifetimes,Generics and Modules

## About the Project

This lab focuses on working with Lifetimes, Generics and Modules in Rust. These are essential tools for ensuring that references never outlive the data they point to.


## Learning Objectives

After completing this lab, I was able to:

read and write lifetime annotations on funtion signatures and structs
understand why the computer rejects refernces that can dangle
write generic funtions constrained by trait boundsusing where clauses
know the difference between pub, pub(crate) and private items
use pub use to re-export items.

## Project Structure

.
├── Cargo.toml
├── Cargo.lock
├── src
│   └── main.rs
        exercise_1.rs
        exercise_2.rs
    utils.rs
    geometry/
        mod.rs
        shapes.rs
        transforms.rs
└── README.md

## Requirements

Before running the project, make sure Rust is installed on your computer.

You can verify your installation with:

bash
rustc --version
cargo --version


## Building the Project

Compile the project by running:

```bash
cargo build

```
## Running the Program

Execute the program with:

bash
cargo run

## Technologies Used

- Rust
- Cargo
- Visual Studio Code

## License

This project was created as part of a Rust programming laboratory exercise for educational purposes.