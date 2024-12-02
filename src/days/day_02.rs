use std::fs;

pub fn run(file_path: &str) {
    let contents = fs::read_to_string(file_path).expect("Failed to read file");
    let reports_str = contents.split_terminator('\n');

    let reports: Vec<Vec<u8>> = reports_str
        .map(|report| {
            report
                .split_whitespace()
                .map(|level| level.parse().expect("Invalid level"))
                .collect()
        })
        .collect();

    let is_increasing = |report: Vec<u8>| -> bool {
        let increase = report
            .windows(2)
            .map(|s| (s[0], s[1]))
            .map(|(x, y)| x < y)
            .all(|x| x);

        increase
    };

    let is_decreasing = |report: Vec<u8>| -> bool {
        let decrease = report
            .windows(2)
            .map(|s| (s[0], s[1]))
            .map(|(x, y)| x > y)
            .all(|x| x);

        decrease
    };

    let is_in_range = |report: Vec<u8>| -> bool {
        let safe = report
            .windows(2)
            .map(|s| (s[0], s[1]))
            .all(|(x, y)| x.abs_diff(y) >= 1 && x.abs_diff(y) <= 3);
        safe
    };

    let num_safe_reports: usize = reports
        .iter()
        .filter(|&report| {
            is_in_range(report.clone())
                && (is_increasing(report.clone()) || is_decreasing(report.clone()))
        })
        .count();

    println!("Number of Safe Reports: {}", num_safe_reports)
}
