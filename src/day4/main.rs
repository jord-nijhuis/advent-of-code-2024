use std::env;
use std::fs::read_to_string;

struct Direction {
    x: i32,
    y: i32,
}

const LEFT: Direction = Direction { x: -1, y: 0 };
const RIGHT: Direction = Direction { x: 1, y: 0 };

const TOP: Direction = Direction { x: 0, y: -1 };
const BOTTOM: Direction = Direction { x: 0, y: 1 };

const TOP_LEFT: Direction = Direction { x: -1, y: -1 };
const TOP_RIGHT: Direction = Direction { x: 1, y: -1 };
const BOTTOM_LEFT: Direction = Direction { x: -1, y: 1 };
const BOTTOM_RIGHT: Direction = Direction { x: 1, y: 1 };

fn get_table(file: &str) -> Vec<Vec<char>> {
    read_to_string(file)
        .expect("File not found")
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn is_match_in_direction(
    table: &Vec<Vec<char>>,
    word: &str,
    x: usize,
    y: usize,
    direction: &Direction,
) -> bool {
    let mut x = x as i32;
    let mut y = y as i32;

    for char in word.chars() {
        if x < 0 || x >= table.len() as i32 {
            return false;
        }

        if y < 0 || y >= table[0].len() as i32 {
            return false;
        }

        if table[x as usize][y as usize] != char {
            return false;
        }

        x += direction.x;
        y += direction.y;
    }

    true
}

fn is_match_xmas(table: &Vec<Vec<char>>, x: usize, y: usize) -> usize {
    let directions = vec![
        LEFT,
        RIGHT,
        TOP,
        BOTTOM,
        TOP_LEFT,
        TOP_RIGHT,
        BOTTOM_LEFT,
        BOTTOM_RIGHT,
    ];

    let word = "XMAS";

    directions
        .iter()
        .filter(|direction| {
            if is_match_in_direction(table, word, x, y, direction) {
                return true;
            }

            false
        })
        .count()
}

fn find_xmas(table: &Vec<Vec<char>>) -> u32 {
    let mut occurrence = 0;

    for x in 0..table.len() {
        for y in 0..table[0].len() {
            occurrence += is_match_xmas(table, x, y)
        }
    }

    occurrence as u32
}

fn is_match_mas(table: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let word = "MAS";
    let offset = word.len() - 1;

    let right_to_left = is_match_in_direction(table, "MAS", x, y, &BOTTOM_RIGHT)
        || is_match_in_direction(table, "MAS", x + offset, y + offset, &TOP_LEFT);
    let left_to_right = is_match_in_direction(table, "MAS", x + offset, y, &BOTTOM_LEFT)
        || is_match_in_direction(table, "MAS", x, y + offset, &TOP_RIGHT);

    right_to_left && left_to_right
}

fn find_mas(table: &Vec<Vec<char>>) -> u32 {
    let mut occurrence = 0;

    for x in 0..table.len() {
        for y in 0..table[0].len() {
            if is_match_mas(table, x, y) {
                occurrence += 1
            }
        }
    }

    occurrence as u32
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let table = get_table(input);

    let occurrence = find_xmas(&table);

    println!("Occurrence of XMAS in the table: {}", occurrence);

    let occurrence = find_mas(&table);

    println!("Occurrence of MAS in the table: {}", occurrence);
}
