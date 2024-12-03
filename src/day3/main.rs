use std::env;
use std::fs::read_to_string;
use regex::Regex;

fn get_memory(input: &str) -> String {
    read_to_string(input).expect("File not found")
}

fn get_multiplication_result(memory: &str) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    re.captures_iter(memory)
        .map(|cap| {
            let a = cap[1].parse::<i32>().unwrap();
            let b = cap[2].parse::<i32>().unwrap();
            a * b
        })
        .fold(0, |acc, x| acc + x)
}


fn remove_conditionals(memory: &str) -> String {
    // Remove all the code between the first `don't()` and the next `do()` or the end of the string
    let re = Regex::new(r"(?Us)don't\(\).*(do\(\)|$)").unwrap();

    re.replace_all(memory, "").to_string()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let memory = get_memory(input);

    let multiplication_result = get_multiplication_result(&memory);
    println!("Multiplication result: {}", multiplication_result);

    let multiplication_result_conditionals = get_multiplication_result(&remove_conditionals(&memory));
    println!("Multiplication result with conditionals: {}", multiplication_result_conditionals);
}