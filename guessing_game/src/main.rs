
// We want input from the user
use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    // thread range means use the current thread which is seeded
    let secret = rand::thread_rng().gen_range(1..=100); // spread from 1..=100
    
    // Loop literally creates an infinite loop
    loop {
        println!("Guess a number: ");
        let mut guess = String::new(); // Make a new string and store in a mutable var
        
        io::stdin()
        .read_line(&mut guess) // Use readline and assign it to a reference of guess
        .expect("Failed, whoops@"); // Provide a failure message, like catch
        
        // Use shadowing to typecast guess to a number so we can compare it
        // parse is needed to do the actual type conversion
        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num, // Number
            Err(_) => {
                println!("That wasn't a number, was it?");
                continue;
            }, // Not a number, ask again
        }; 
        
        println!("You guessed: {guess}");
        
        match  guess.cmp(&secret) {
            Ordering::Less => println!("Na, mate, too low!"),
            Ordering::Equal => {
                println!("Alright, look at you, Mystic Meg!");
                break;
            },
            Ordering::Greater => println!("Na, mate, too high!")
        }
    }
}
