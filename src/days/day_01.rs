use std::collections::HashMap;
use std::fs;

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

        even.sort_unstable();
        odd.sort_unstable();

        (even, odd)
    };

    let total_distance: u32 = location_list_one
        .iter()
        .zip(location_list_two.iter())
        .map(|(x, y)| x.abs_diff(*y))
        .sum();

    println!("Total Distance: {}", total_distance);

    let mut location_map: HashMap<u32, u32> = HashMap::new();
    for &location in &location_list_two {
        *location_map.entry(location).or_insert(0) += 1;
    }

    let similarity_score: u32 = location_list_one
        .iter()
        .map(|&location_id| location_id * location_map.get(&location_id).unwrap_or(&0))
        .sum();

    println!("Similarity Score: {}", similarity_score);
}
