use std::fs;

use regex::Regex;

pub fn run(file_path: &str) {
    let contents = fs::read_to_string(file_path).expect("Failed to read file");

    let re = Regex::new(r"(?m)mul\((?<num1>\d+),(?<num2>\d+)\)").unwrap();
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

    let re = Regex::new(r"(?m)(?<do>do\(\))|(?<dont>don't\(\))|mul\((?<num1>\d+),(?<num2>\d+)\)")
        .unwrap();
    let captures = re.captures_iter(&contents);

    let captures_list = captures.fold(Vec::new(), |mut list, element| {
        if let Some(value) = element.name("do").map(|m| m.as_str()) {
            list.push(value);
        } else if let Some(value) = element.name("dont").map(|m| m.as_str()) {
            list.push(value);
        } else {
            list.push(element.name("num1").map(|m| m.as_str()).unwrap());
            list.push(element.name("num2").map(|m| m.as_str()).unwrap());
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
                    if mul_flag && !num_flag {
                        let new_num = element.parse::<i32>().unwrap();
                        (acc, mul_flag, new_num, true)
                    } else if mul_flag && num_flag {
                        let new_acc = acc + num * element.parse::<i32>().unwrap();
                        (new_acc, mul_flag, 0, false)
                    } else {
                        (acc, mul_flag, num, num_flag)
                    }
                }
            },
        )
        .0;

    println!("Multiplication Results (Do's & Don't's): {}", mul_results);
}
