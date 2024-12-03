use std::fs;

use regex::Regex;

pub fn run(file_path: &str) {
    let contents = fs::read_to_string(file_path).expect("Failed to read file");

    let re = Regex::new(r"(?m)mul\((\d+),(\d+)\)").unwrap();
    let captures = re.captures_iter(&contents);

    let factors = captures.map(|mat| {
        (
            mat.get(1)
                .expect("Invalid capture group")
                .as_str()
                .parse::<u32>()
                .expect("Invalid number"),
            mat.get(2)
                .expect("Invalid capture group")
                .as_str()
                .parse::<u32>()
                .expect("Invalid number"),
        )
    });

    let mul_results: u32 = factors.map(|(x, y)| x * y).sum();

    println!("Multiplication Results: {}", mul_results);
}
