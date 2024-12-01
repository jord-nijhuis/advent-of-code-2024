use std::fs::read_to_string;
use std::env;

fn get_lists(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut first_list = Vec::new();
    let mut second_list = Vec::new();

    for line in read_to_string(input).unwrap().lines() {
        let mut split = line.split_whitespace();
        first_list.push(split.next().unwrap().parse().unwrap());
        second_list.push(split.next().unwrap().parse().unwrap());
    }

    first_list.sort();
    second_list.sort();

    (first_list, second_list)
}

fn get_distance(first_list: &Vec<i32>, second_list: &Vec<i32>) -> i32 {
    first_list
        .iter()
        .zip(second_list.iter())
        .fold(0, |acc, (first, second)| {
            acc + (second - first).abs()
        })
}

fn get_similarity(first_list: &Vec<i32>, second_list: &Vec<i32>) -> i32 {
    first_list
        .iter()
        .fold(0, |acc, x| {
            acc + x * second_list.iter().filter(|&y| y == x).count() as i32
        })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let (first_list, second_list) = get_lists(input);

    let distance = get_distance(&first_list, &second_list);
    println!("Distance: {}", distance);

    let similarity = get_similarity(&first_list, &second_list);
    println!("Similarity: {}", similarity);
}
