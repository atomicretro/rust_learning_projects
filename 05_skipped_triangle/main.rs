use std::io;

fn main() {
    println!("How many stars?");
    let mut max = String::new();
    io::stdin().read_line(&mut max).expect("Failed to read line");
    let max: u16 = max.trim().parse().expect("Not a number!");

    for n in 1..=max {
        if n % 2 == 0 {
          continue;
        }

        let stars = "*".repeat(n.into());
        println!("{}", stars);
    }
}
