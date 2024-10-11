use std::{io, str::FromStr, f32::consts::PI};

enum CircleParts {
  Area,
  Diameter,
  Radius,
}

impl FromStr for CircleParts {
  type Err = ();
  fn from_str(input: &str) -> Result<CircleParts, Self::Err> {
    match &input.to_lowercase()[..] {
      "area" => Ok(CircleParts::Area),
      "diameter" => Ok(CircleParts::Diameter),
      "radius" => Ok(CircleParts::Radius),
      _ => Err(()),
    }
  }
}

fn main() {
  println!("Let's calculate you a circle.");
  println!("Do you know the Area, Diameter, or Radius?");
  let mut part = String::new();
  io::stdin().read_line(&mut part).expect("Failed to read line");
  let part: CircleParts = CircleParts::from_str(part.trim()).unwrap();

  println!("Dope. Now what's the value of that bad boy?");
  let mut value = String::new();
  io::stdin().read_line(&mut value).expect("Failed to read line");
  let value: f32 = value.trim().parse().expect("Not a number!");

  match part {
    CircleParts::Area => {
      let radius = (value / PI).sqrt();
      let diameter = radius * 2.0;
      print_answer(value, diameter, radius);
    },
    CircleParts::Diameter => {
      let radius = value / 2.0;
      let area = PI * radius.powf(2.0);
      print_answer(area, value, radius);
    },
    CircleParts::Radius => {
      let area = PI * value.powf(2.0);
      let diameter = value * 2.0;
      print_answer(area, diameter, value);
    },
  }
}

fn print_answer(area: f32, diameter: f32, radius: f32) {
  println!("The Area is: {area}");
  println!("The Diameter is: {diameter}");
  println!("The Radius is: {radius}");
}
