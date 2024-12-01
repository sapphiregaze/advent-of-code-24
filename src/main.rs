use std::env;

mod days;

fn main() {
    let day = env::args().nth(1).and_then(|s| s.parse::<i8>().ok());

    match day {
        Some(1) => days::day_01::run(),
        Some(n) => println!("Day {} not implemented", n),
        _ => println!("invalid input."),
    }
}
