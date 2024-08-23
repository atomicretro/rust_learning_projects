use std::io;
use rand::Rng;

fn main() {
    let target_num = rand::thread_rng().gen_range(1..=100);
    
    println!("Welcome to Alec's FAN-TAB-ULOUS guessing extravaganza!");
    println!("We've picked out a special, SECRET number just for you.");
    println!("It's a number between 1 and 100.");
    println!("So... guess it already.");

    let mut done = false;

    while !done {
        let mut guess_num = String::new();
        io::stdin().read_line(&mut guess_num).expect("Failed to read line");
        let guess_num: u8 = guess_num.trim().parse().expect("WRONG. You must guess a positive, whole number.");
        if guess_num > 100 {
            panic!("\nWRONG. You tried to guess over 100... you sneaky old goat");
        }

        if guess_num > target_num {
            println!("So close! But no, not really. You guessed too high.");
        } else if guess_num < target_num {
            println!("Whoa! Amazing! But no, not really. You guessed too low.")
        } else {
            println!("YOU DID IT! YOU'RE RELEASED FROM THIS NEVER ENDING PURGATORY OF MATH AND FUN!");
            done = true;
        }
    }

    println!("I'm proud of you, bud.");
}
