use std::convert::TryFrom;
use std::fs;
use std::iter::zip;

pub fn run(file_path: &str) {
    let contents = fs::read_to_string(file_path).expect("Failed to read file");
    let location_ids: Vec<&str> = contents.split_whitespace().collect();

    let (location_list_one, location_list_two): (Vec<u32>, Vec<u32>) = {
        let (mut even, mut odd) = location_ids.iter().enumerate().fold(
            (Vec::new(), Vec::new()),
            |(mut even, mut odd), (i, element)| {
                if i % 2 == 0 {
                    even.push(element.parse().unwrap());
                } else {
                    odd.push(element.parse().unwrap());
                }
                (even, odd)
            },
        );

        even.sort();
        odd.sort();

        (even, odd)
    };

    let total_distance: u32 = zip(location_list_one.clone(), location_list_two.clone())
        .map(|(x, y)| x.abs_diff(y))
        .sum();

    println!("Total Distance: {}", total_distance);

    let similarity_score: u32 = location_list_one
        .iter()
        .map(|location_id| {
            location_id
                * u32::try_from(
                    location_list_two
                        .iter()
                        .filter(|&element| *element == *location_id)
                        .count(),
                )
                .unwrap()
        })
        .sum();

    println!("Similarity Score: {}", similarity_score);
}
