use std::io;

fn main() {
    println!("Input a positive whole number to get its factorial.");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input: u32 = input.trim().parse().expect("Not a positive whole number!");
    if input == 0 {
      panic!("Zero doesn't count.");
  }

    let loop_factorial = factorial_loop(input);
    let recur_factorial = factorial_recursion(input);

    println!("Through looping we got: {loop_factorial}.");
    println!("Through recursion we got: {recur_factorial}.");
}

fn factorial_loop(input: u32) -> u32 {
    let mut factorial: u32 = 1;
    for n in 2..=input {
        factorial = factorial * n;
    }

    factorial
}

fn factorial_recursion(input: u32) -> u32 {
    if input == 1 {
      return 1;
    }

    return input * factorial_recursion(input - 1);
}
