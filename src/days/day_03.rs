use regex::{Captures, Regex};
use std::fs;

pub fn run(file_path: &str) {
    let contents = fs::read_to_string(file_path).expect("Failed to read file");

    let re = Regex::new(r"(?m)mul\((?<num1>\d{1,3}),(?<num2>\d{1,3})\)").unwrap();
    let captures = re.captures_iter(&contents);

    let extract_and_parse = |capture: &Captures<'_>, i: usize| -> u32 {
        capture
            .get(i)
            .expect("Invalid capture group")
            .as_str()
            .parse::<u32>()
            .expect("Invalid number")
    };

    let factors = captures.map(|capture| {
        (
            extract_and_parse(&capture, 1),
            extract_and_parse(&capture, 2),
        )
    });

    let mul_results: u32 = factors.map(|(x, y)| x * y).sum();

    println!("Multiplication Results: {}", mul_results);

    let re = Regex::new(
        r"(?m)(?<do>do\(\))|(?<dont>don't\(\))|mul\((?<num1>\d{1,3}),(?<num2>\d{1,3})\)",
    )
    .unwrap();
    let captures = re.captures_iter(&contents);

    let captures_list = captures.fold(Vec::new(), |mut list, element| {
        if let Some(value) = element
            .name("do")
            .or_else(|| element.name("dont"))
            .map(|m| m.as_str())
        {
            list.push(value);
        } else {
            list.extend(
                ["num1", "num2"]
                    .iter()
                    .filter_map(|&name| element.name(name).map(|m| m.as_str())),
            );
        }
        list
    });

    let mul_results = captures_list
        .iter()
        .fold(
            (0, true, 0, false),
            |(acc, mul_flag, num, num_flag), &element| match element {
                "do()" => (acc, true, num, num_flag),
                "don't()" => (acc, false, num, num_flag),
                _ => {
                    if !mul_flag {
                        return (acc, mul_flag, num, num_flag);
                    }
                    if !num_flag {
                        let new_num = element.parse::<u32>().unwrap();
                        return (acc, mul_flag, new_num, true);
                    }
                    let new_acc = acc + num * element.parse::<u32>().unwrap();
                    (new_acc, mul_flag, 0, false)
                }
            },
        )
        .0;

    println!("Multiplication Results (Do's & Don't's): {}", mul_results);
}
