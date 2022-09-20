use std::io;
use rand::Rng;
use std::cmp::Ordering;

// Main function in rust
fn main() {
    let secret = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret}");

    loop {
        println!("Input your guess: ");
        let mut guess = String::new(); // Muteable variable reading a String

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read_line");

        // Converting guees into a num variable
        let guess: u32 = guess.trim().parse() {
            ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");
        // Comparing
        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You Win!!!");
                break;
            },
        }

    }
}
