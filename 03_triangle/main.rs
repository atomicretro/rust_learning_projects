use std::io;

fn main() {
    println!("How many stars?");
    let mut max = String::new();
    io::stdin().read_line(&mut max).expect("Failed to read line");
    let max: u16 = max.trim().parse().expect("Not a number!");

    let mut idx: u16 = max;
    loop {
        if idx <= 0 {
          break;
        }

        let stars = collect_stars(idx);
        println!("{stars}");
        idx -= 1;
    }
}

fn collect_stars(num: u16) -> String {
    let mut stars = String::from("");
    let mut idx: u16 = 0;
    loop {
        if idx >= num {
          break;
        }

        stars.push_str("*");
        idx += 1;
    }

    stars
}
