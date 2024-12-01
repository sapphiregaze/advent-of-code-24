use std::env;

mod days;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (day, input_file) = parse_config(&args);

    match day {
        1 => days::day_01::run(input_file),
        _ => println!("invalid input."),
    }
}

fn parse_config(args: &[String]) -> (u8, &str) {
    let day = args[1].parse().unwrap();
    let input_file = &args[2];

    (day, input_file)
}
