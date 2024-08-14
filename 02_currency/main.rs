use std::io;
use std::collections::HashMap;

fn main() {
    let currencies = init_currencies();

    println!("Hello! Welcome to the money changer.");
    println!("Please input your current wealth in the following format: 123 ABC");
    println!("Accepted currencies are COP, HKD, USD.");

    let mut wealth_input = String::new();
    io::stdin()
        .read_line(&mut wealth_input)
        .expect("Failed to read line");
    let wealth_input: Vec<&str> = wealth_input.trim().split(" ").collect();

    if wealth_input.len() != 2 {
        panic!("\nWRONG. Please input your wealth in the correct format, e.g. 10 USD");
    }
    let wealth_val: f64 = wealth_input[0].parse().expect("\nWRONG. Please input a number as the first value.");
    let wealth_cur = currencies[wealth_input[1]];

    println!("\nPhenomenal.");

    println!("\nPlease input the currency you want to convert your welath to.");
    let mut convert_input = String::new();
    io::stdin()
        .read_line(&mut convert_input)
        .expect("Failed to read line");
    let convert_input = convert_input.trim();

    let convert_cur = currencies[convert_input];

    println!("\nPhenomenal.");
    println!("\nCalculating...");

    let total: f64 = wealth_val * (convert_cur / wealth_cur);

    println!("\nWell! You have {total} {convert_input}. Good job!");

}

fn init_currencies() -> HashMap<String, f64> {
    let mut currencies = HashMap::new();

    currencies.insert(String::from("COP"), 3000.0);
    currencies.insert(String::from("HKD"), 7.75);
    currencies.insert(String::from("USD"), 1.0);

    currencies
}
