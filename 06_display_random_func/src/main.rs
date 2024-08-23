use rand::Rng;

fn main() {
    let num = rand::thread_rng().gen_range(1..=3);
    match num {
        1 => println!("fn hello_world"),
        2 => println!("fn goodbye_world"),
        3 => println!("fn FIGHT_ME_WORLD"),
        _ => (),
    };
}
