use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::read_to_string;

fn get_input(input: &str) -> (HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>) {
    let mut rules = HashMap::new();
    let mut prints = Vec::new();

    for line in read_to_string(input).expect("File not found").lines() {
        if line.is_empty() {
            continue;
        }

        if line.contains("|") {
            let split = line
                .split("|")
                .map(|x| x.parse().expect("Invalid number"))
                .collect::<Vec<i32>>();

            rules
                .entry(split[0])
                .or_insert(HashSet::new())
                .insert(split[1]);

            continue;
        }

        prints.push(
            line.split(',')
                .map(|x| x.parse().expect("Invalid number"))
                .collect::<Vec<i32>>(),
        );
    }

    (rules, prints)
}

fn get_invalid_print_order_index(
    print: &Vec<i32>,
    rules: &HashMap<i32, HashSet<i32>>,
) -> Option<(usize, usize)> {
    for (index, page) in print.iter().enumerate() {
        // This page should not necessarily be printed before another page
        if rules.get(page).is_none() {
            continue;
        }

        // Get the page that should be printed after this page
        let pages_after_this_page = rules.get(page).unwrap();

        for page_after_this_page in pages_after_this_page {
            if print
                .iter()
                .position(|x| x == page_after_this_page)
                .is_none()
            {
                continue;
            }

            let index_page_after = print
                .iter()
                .take(index)
                .position(|x| x == page_after_this_page);

            // The page that should be printed after this page is printed before this page
            if !index_page_after.is_none() && index_page_after.unwrap() < index {
                return Some((index, index_page_after.unwrap()));
            }
        }
    }

    None
}

fn is_valid_print_order(print: &Vec<i32>, rules: &HashMap<i32, HashSet<i32>>) -> bool {
    get_invalid_print_order_index(print, rules).is_none()
}

fn get_correct_prints(prints: &Vec<Vec<i32>>, rules: &HashMap<i32, HashSet<i32>>) -> Vec<Vec<i32>> {
    prints
        .iter()
        .filter(|print| is_valid_print_order(print, rules))
        .map(|x| x.clone())
        .collect()
}

fn get_incorrect_prints<'a>(
    prints: &'a Vec<Vec<i32>>,
    rules: &HashMap<i32, HashSet<i32>>,
) -> Vec<&'a Vec<i32>> {
    prints
        .iter()
        .filter(|print| !is_valid_print_order(print, rules))
        .collect()
}

fn fix_incorrect_print(print: Vec<i32>, rules: &HashMap<i32, HashSet<i32>>) -> Vec<i32> {
    let mut print = print;

    while let Some((i, j)) = get_invalid_print_order_index(&print, rules) {
        print.swap(i, j);
    }

    print
}

fn fix_incorrect_prints(
    prints: Vec<&Vec<i32>>,
    rules: &HashMap<i32, HashSet<i32>>,
) -> Vec<Vec<i32>> {
    prints
        .iter()
        .map(|print| fix_incorrect_print((*print).clone(), rules))
        .collect()
}

fn get_middle_pages(prints: &Vec<Vec<i32>>) -> i32 {
    prints
        .iter()
        .map(|print| print.get(print.len() / 2).unwrap())
        .sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let (rules, prints) = get_input(input);

    let correct_prints = get_correct_prints(&prints, &rules);

    let middle_pages = get_middle_pages(&correct_prints);
    println!("Middle pages: {}", middle_pages);

    let incorrect_prints = get_incorrect_prints(&prints, &rules);
    let fixed_prints = fix_incorrect_prints(incorrect_prints, &rules);

    let middle_pages = get_middle_pages(&fixed_prints);
    println!("Fixed middle pages: {}", middle_pages);
}
