use std::fs;

pub fn run(file_path: &str) {
    let contents = fs::read_to_string(file_path).expect("Failed to read file");
    let location_ids: Vec<&str> = contents.split_whitespace().collect();

    let mut location_list_one: Vec<u32> = Vec::new();
    let mut location_list_two: Vec<u32> = Vec::new();

    for (i, &location_id) in location_ids.iter().enumerate() {
        if i % 2 == 0 {
            location_list_one.push(location_id.parse().unwrap());
        } else {
            location_list_two.push(location_id.parse().unwrap());
        }
    }

    location_list_one.sort();
    location_list_two.sort();

    let mut total_distance: u32 = 0;
    for (i, &location_id) in location_list_one.iter().enumerate() {
        total_distance += location_id.abs_diff(location_list_two[i]);
    }

    println!("{}", total_distance);
}
