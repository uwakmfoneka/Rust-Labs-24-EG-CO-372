# Rust Lab: Collections and Iterators

## About the Project

This lab focuses on working with **collections** and **iterators** in Rust. These are essential tools for storing, accessing, and processing groups of data efficiently.

The program demonstrates how to use common collection types such as vectors and hash maps, as well as how iterators can simplify tasks like searching, filtering, and transforming data. The exercises are designed to build a better understanding of Rust's approach to handling collections while writing clean and readable code.

## Learning Objectives

After completing this lab, I was able to:

- push, sort, pop and slice a vec.
- understand .entry().or_insert() hash map updates.
- chain filter, map, take, and collect
- Use iterator adapters such as `map()` and `filter()`.
- Collect iterator results into new collections.
- Write cleaner and more efficient code using iterators instead of manual loops.

## Project Structure

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

## Topics Covered

### Vectors

Vectors are dynamic arrays that can grow or shrink as needed. They are one of the most commonly used collection types in Rust.


### Hash Maps

Hash maps store data as key-value pairs, making it easy to look up values using a key.

Example:

rust
use std::collections::HashMap;

let mut grades = HashMap::new();

grades.insert("Alice", 90);
grades.insert("Bob", 85);

println!("{:?}", grades);


---

### Iterators

Iterators allow you to process collections one element at a time without writing complex loops.

### Iterator Adapters

Rust provides several useful iterator methods that make data processing easier.

Using `map():

rust
let doubled: Vec<i32> = numbers.iter()
    .map(|x| x * 2)
    .collect();


Using `filter():

rust
let even_numbers: Vec<i32> = numbers.iter()
    .filter(|x| **x % 2 == 0)
    .cloned()
    .collect();


These methods help write concise and readable code without manually managing loops.

## What I Learned

This lab helped me understand how Rust manages collections and how iterators make working with data much easier. Instead of writing long loops, iterator methods like `map()` and `filter()` allow data to be processed in a more readable and efficient way. I also learned when to use vectors for ordered data and hash maps for storing information as key-value pairs.

Overall, this exercise gave me more confidence in handling collections and writing cleaner Rust code.

## Technologies Used

- Rust
- Cargo
- Visual Studio Code

## Possible Improvements

Some features that could be added in the future include:

- Working with `HashSet`
- Sorting collections
- Reading collection data from a file
- Accepting user input to populate collections
- Using custom structs inside vectors and hash maps
- Exploring more advanced iterator methods such as `fold()`, `find()`, and `enumerate()`

## License

This project was created as part of a Rust programming laboratory exercise for educational purposes.