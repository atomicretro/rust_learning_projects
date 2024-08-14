use std::io;

fn main() {
    println!("Let's get fibby:");
    let mut max = String::new();
    io::stdin().read_line(&mut max).expect("Failed to read line");
    let max: u16 = max.trim().parse().expect("Not a number!");

    let mut fib_vec = vec![1, 1];

    loop {
        let next_fib = fib_vec[fib_vec.len() - 2] + fib_vec[fib_vec.len() - 1];
        if next_fib >= max {
            break;
        }

        fib_vec.push(next_fib);
    }

    println!("{:?}", fib_vec);
}
