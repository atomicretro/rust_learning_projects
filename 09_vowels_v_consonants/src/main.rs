use std::io;
use std::collections::HashMap;

fn main() {
    let vowels = get_vowels();
    let consonants = get_consonants();

    println!("Give me a string and I'll count your vowels, I'll count your consonants,");
    println!("but I WILL NOT count your damn NON-ALPHABETIC CHARACTERS.");
    println!("Get those jabronis OUTTA HERE!\n");

    let mut input_str = String::new();
    io::stdin().read_line(&mut input_str).expect("Failed to read line");
    let input_str: Vec<&str> = input_str.trim().split("").collect();

    let mut vowel_count = 0;
    let mut consonant_count = 0;
    for letter in input_str {
        if letter == "" || letter == " " {
            // ignore empty characters and spaces
            continue;
        }

        let mut should_quit = 0;
        match vowels.get(letter) {
            Some(_) => vowel_count += 1,
            None => should_quit += 1,
        }
        match consonants.get(letter) {
            Some(_) => consonant_count += 1,
            None => should_quit += 1,
        }

        if should_quit >= 2 {
            println!("\nI TOLD YOU NOT TO INCLUDE ANY NON-ALPHABETIC CHARACTERS!");
            println!("NOW I'M FUMING! I GOTTA GO LIE DOWN!");
            println!("I counted everything up until \"{}\".", letter);
            println!("You disgust me.");
            break;
        }
    }

    println!("\nvowels counted: {}", vowel_count);
    println!("consonants counted: {}", consonant_count);
}

fn get_vowels() -> HashMap<&'static str, bool> {
    HashMap::from([
        ("a", true), ("e", true), ("i", true), ("o", true), ("u", true),
    ])
}

fn get_consonants() -> HashMap<&'static str, bool> {
    HashMap::from([
        ("b", true), ("c", true), ("d", true), ("f", true), ("g", true),
        ("h", true), ("j", true), ("k", true), ("l", true), ("m", true),
        ("n", true), ("p", true), ("q", true), ("r", true), ("s", true),
        ("t", true), ("v", true), ("w", true), ("x", true), ("y", true),
        ("z", true),
    ])
}
