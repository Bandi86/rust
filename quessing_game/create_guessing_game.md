# How to learn Rust?

source: https://doc.rust-lang.org/

# Installation:

Installing rustup on Windows
On Windows, go to https://www.rust-lang.org/tools/install and follow the instructions for installing Rust. At some point in the installation, you’ll be prompted to install Visual Studio. This provides a linker and the native libraries needed to compile programs. If you need more help with this step, see https://rust-lang.github.io/rustup/installation/windows-msvc.html

The rest of this book uses commands that work in both cmd.exe and PowerShell. If there are specific differences, we’ll explain which to use.

Troubleshooting
To check whether you have Rust installed correctly, open a shell and enter this line:

`$ rustc --version`

runing files:
rustc main.rs

cargo:
create project with cargo new [project name]
enter the folder

# Building and Running Cargo Project

`cargo build`

run:
`./target/debug/hello_cargo`
wee got the hello world text

We just built a project with cargo build and ran it with ./target/debug/hello_cargo, but we can also use cargo run to compile the code and then run the resultant executable all in one command:
`cargo run`

We can build a project without producing a binary to check for errors using: `cargo check`

# Programming a Guessing Game

create a project file and run after that edit the src/main.rs

We need to bring the io input/output library into scope. The io library comes from the standard library, known as std:

`use std::io;`
By default, Rust has a set of items defined in the standard library that it brings into the scope of every program. This set is called the prelude, and you can see everything in it in the standard library documentation.

If a type you want to use isn’t in the prelude, you have to bring that type into scope explicitly with a use statement. Using the std::io library provides you with a number of useful features, including the ability to accept user input.

store values:
`let mut guess = String::new();`
let apples = 5; // immutable
let mut bananas = 5; // mutable

Receiving User Input:
Recall that we included the input/output functionality from the standard library with use std::io; on the first line of the program. Now we’ll call the stdin function from the io module, which will allow us to handle user input:

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

adding random number
cargo.toml 
[dependencies]
rand = "0.8.5"
cargo build

`use rand::Rng;`
`let secret_number = rand::thread_rng().gen_range(1..=100);`
`println!("The secret number is: {}", secret_number);`

compare the numbers:
The Ordering type is another enum and has the variants Less, Greater, and Equal. These are the three outcomes that are possible when you compare two values.
`use std::cmp::Ordering;`

```rust
// comparing numbers
match guess.cmp(&secret_number) {
   Ordering::Less => println!("Too small!"),
   Ordering::Greater => println!("Too big!"),
   Ordering::Equal => println!("You win!"),
}
```

Allowing Multiple Guesses with Looping
The loop keyword creates an infinite loop. We’ll add a loop to give users more chances at guessing the number:
