# Rust Programming Lab Exercises

## Overview

This repository contains solutions to Rust programming lab exercises focused on concurrency, asynchronous programming, and file input/output (File I/O). The exercises demonstrate how Rust handles safe concurrency, shared memory, message passing, asynchronous tasks, and file operations.

## Topics Covered

- Thread creation using `std::thread`
- Shared state with `Arc<Mutex<T>>`
- Message passing using channels (`mpsc`)
- File reading and writing
- Recursive directory traversal
- Asynchronous programming with Tokio
- Concurrent execution using `tokio::join!`

## Exercises

### Exercise A – Spawning Threads
- Created and managed multiple threads.
- Computed the sum of different ranges concurrently.
- Joined all threads and combined the results.

### Exercise B – Shared State
- Used `Arc<Mutex<T>>` to safely share data between threads.
- Improved performance by using a local accumulator before updating the shared counter.

### Exercise C – Channels (Message Passing)
- Used Rust channels (`mpsc`) for communication between threads.
- Sent either successful results or error messages.
- Processed messages in the receiver using pattern matching.

### Exercise D – File I/O
- Created and wrote data to a log file.
- Counted the number of lines in the file.
- Filtered log entries containing errors.
- Recursively listed all `.rs` files in a directory.

### Exercise E – Async/Await with Tokio
- Simulated asynchronous data fetching.
- Compared sequential and concurrent execution.
- Used `tokio::join!` to execute multiple asynchronous tasks simultaneously.

## Requirements

- Rust (latest stable version)
- Cargo
- Tokio crate

## Installing Dependencies