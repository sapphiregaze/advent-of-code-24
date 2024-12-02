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

    let is_sorted_by = |report: &Vec<u8>, cmp: fn(u8, u8) -> bool| -> bool {
        report.windows(2).all(|pair| cmp(pair[0], pair[1]))
    };

    let is_increasing = |report: &Vec<u8>| is_sorted_by(report, |x, y| x < y);
    let is_decreasing = |report: &Vec<u8>| is_sorted_by(report, |x, y| x > y);
    let is_in_range = |report: &Vec<u8>| -> bool {
        report
            .windows(2)
            .map(|s| (s[0], s[1]))
            .all(|(x, y)| x.abs_diff(y) >= 1 && x.abs_diff(y) <= 3)
    };

    let is_safe = |report: &Vec<u8>| -> bool {
        (is_increasing(report) || is_decreasing(report)) && is_in_range(report)
    };

    let num_safe_reports: usize = reports.iter().filter(|&report| is_safe(report)).count();

    println!("Number of Safe Reports: {}", num_safe_reports);

    // Generates all possible permutations of report when a single level is removed
    let report_permutations = |report: &Vec<u8>| -> Vec<Vec<u8>> {
        let mut permutations: Vec<Vec<u8>> = Vec::new();
        for i in 0..report.len() {
            let mut modified = report.clone();
            modified.remove(i);
            permutations.push(modified);
        }
        permutations
    };

    // For each report, if any of the generated permutations is considered as
    // safe, then that report is considered as safe
    let mut num_safe_reports = 0;
    for i in 0..reports.len() {
        if (report_permutations(&reports[i]))
            .iter()
            .map(|report| is_safe(&report))
            .any(|x| x)
        {
            num_safe_reports += 1;
        }
    }

    println!(
        "Number of Safe Reports (Problem Dampener): {}",
        num_safe_reports
    );
}
