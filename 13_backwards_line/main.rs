use std::io;

fn main() {
  println!("We're gonna go BACKWARDS.");
  println!("Give me a sentence, I'll blow your mind.");
  let mut sentence = String::new();
  io::stdin().read_line(&mut sentence).expect("Failed to read line");
  let sentence: &str = sentence.trim();

  println!("{}", do_it_backwards(sentence));
}

fn do_it_backwards(sentence: &str) -> String {
  if sentence.len() == 0 {
    return "".to_string();
  }

  format!(
    "{}{}",
    sentence.chars().last().unwrap().to_string(),
    do_it_backwards(&sentence[0..sentence.len() - 1]),
  )
}
