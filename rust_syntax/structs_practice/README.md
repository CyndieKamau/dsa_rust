# User Profile Program in Rust

# Introduction

This directory provides a simple demonstration of using structs in Rust to represent and manipulate user profile information.

This project was created as a learning exercise to better understand data structures and algorithms in Rust.

# About the Program;

The `UserProfile` program takes user inputs for a user's name, email, and age.

The information is then stored within a struct, and displayed back to the user. 

It serves as a basic introduction to Rust's struct system, and can be used as a starting point for more complex data structures and functionalities.


## Features

* Interactive User Input

* Struct implementation with methods

* Error handling in Rust

* Continuous input loop until `exit` command

* Display of all user info once program halts

# Getting Started

Make sure you have Rust, Cargo installed in your system.

# Running the Program

```sh
$ git clone https://github.com/CyndieKamau/dsa_rust.git

$ cd dsa_rust && cd structs_practice

$ cargo build

```

Follow the on-screen prompts to input the users' details.

Successful running of program looks this way;

```sh
hp@Cyndie:~/Desktop/dsa_rust/rust_syntax/structs_practice/src$ cargo run
   Compiling structs_practice v0.1.0 (/home/hp/Desktop/dsa_rust/rust_syntax/structs_practice)
warning: fields `username`, `email`, and `age` are never read
 --> src/main.rs:7:5
  |
5 | struct UserProfile {
  |        ----------- fields in this struct
6 |
7 |     username: String,
  |     ^^^^^^^^
8 |     email: String,
  |     ^^^^^
9 |     age: i32,
  |     ^^^
  |
  = note: `UserProfile` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis
  = note: `#[warn(dead_code)]` on by default

warning: `structs_practice` (bin "structs_practice") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.22s
     Running `/home/hp/Desktop/dsa_rust/rust_syntax/structs_practice/target/debug/structs_practice`
Add user info below. Type 'exit' to quit
Type user's name: Cyndie
Type user's email address: cyn@gmail.com
Type user's age: 23
Type user's name: Ken
Type user's email address: ken@gmail.com
Type user's age: 36
Type user's name: John
Type user's email address: jo@gmail.com
Type user's age: 14
Type user's name: exit
All Users
UserProfile { username: "Cyndie", email: "cyn@gmail.com", age: 23 }
UserProfile { username: "Ken", email: "ken@gmail.com", age: 36 }
UserProfile { username: "John", email: "jo@gmail.com", age: 14 }

```

  

