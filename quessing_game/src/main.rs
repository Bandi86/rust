use std::io; // import handle input
use rand::Rng; // import random number creator
use std::cmp::Ordering; // matching the two numbers

fn main() {
    println!("Guess the Number!");
    println!("(Type 'q' to quit at any time)"); // Add instruction for quitting

    let secret_number = rand::thread_rng().gen_range(1..=100); // create a random number between 1-100

    // Optional: Remove this line if you don't want to reveal the number during testing
    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess: ");

        let mut guess_input = String::new(); // Use a different name to avoid confusion before parsing

        io::stdin().read_line(&mut guess_input) // handle user input
            .expect("Failed to read line");

        // Trim the input immediately for checking 'q' and for parsing
        let trimmed_input = guess_input.trim();

        // Check if the user wants to quit *before* trying to parse
        if trimmed_input.eq_ignore_ascii_case("q") { // Check for 'q' (case-insensitive)
            println!("Quitting the game. Goodbye!");
            break; // Exit the loop
        }

        // Now, try to parse the input as a number
        let guess: u32 = match trimmed_input.parse() {
             Ok(num) => num,
             Err(_) => {
                 // Handle invalid number input (but not 'q' which was handled above)
                 println!("'{}' is not a valid number or 'q'. Please try again.", trimmed_input);
                 continue; // Skip the rest of this loop iteration and ask again
             }
         };

        println!("You guessed: {}", guess);

        // compare the two numbers
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // Exit the loop because the guess was correct
            }
        }
    }
}
