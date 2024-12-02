use std::env;
use std::fs::read_to_string;

fn remove_index<T: Clone>(vec: &[T], i: usize) -> Vec<T> {
    vec.iter()
        .enumerate()
        .filter(|&(index, _)| index != i)
        .map(|(_, value)| value.clone())
        .collect()
}

fn get_reports(input: &str) -> Vec<Vec<i32>> {
    read_to_string(input)
        .expect("File not found")
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|element| element.parse().expect("Invalid number"))
                .collect::<Vec<i32>>()
        })
        .collect()
}

fn is_safe(report: &Vec<i32>) -> bool {
    for i in 0..report.len() {
        // Check that the difference between the current and the previous report is between -3 and 3
        // 0 is not allowed
        if i >= 1 {
            let difference = report[i] - report[i - 1];

            if difference < -3 || difference > 3 || difference == 0 {
                return false;
            }
        }

        // Check that the report is not increasing and decreasing at the same time
        if i >= 2 && (report[i] > report[i - 1]) != (report[i - 1] > report[i - 2]) {
            return false;
        }
    }

    true
}

fn get_safe_number_of_reports(reports: &Vec<Vec<i32>>) -> usize {
    reports.iter().filter(|report| is_safe(report)).count()
}

fn get_tolerable_number_of_reports(reports: &Vec<Vec<i32>>) -> usize {
    reports
        .iter()
        .filter(|report| {
            // Remove one value from the report at a time and check if the report is valid
            for i in 0..report.len() {
                if is_safe(&remove_index(report, i)) {
                    return true;
                }
            }

            false
        })
        .count()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let reports = get_reports(input);

    println!("Safe reports: {}", get_safe_number_of_reports(&reports));

    println!(
        "Tolerable reports: {}",
        get_tolerable_number_of_reports(&reports)
    );
}
