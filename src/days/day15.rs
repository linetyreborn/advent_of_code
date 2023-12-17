use std::fs;
use std::collections::HashMap;
use std::cmp;

pub fn run() {
    let input = read_string_from_file("ressources/input15.txt");
    run_hash_algorithm(&input);

}

fn run_hash_algorithm(input: &str) {
    let mut result: i128 = 0;
    for line in input.lines() {
        let steps= parse_string(input);

        for step in steps {
            let mut step_result: i128 = 0;
            let hash_char_map = char_to_ascii_mapping(step);
            step.chars().for_each(|c| {
                step_result = ((step_result + (hash_char_map[&c])as i128) * 17 ) % 256;
            });
            result +=step_result;
        }
    }

    println!("{}", result);
}

fn parse_string(input: &str) -> Vec<&str> {
    input.trim().split(",").collect()
}

fn read_string_from_file(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| panic!("Unable to read file: {}", path))
}

// function that returns hashmap of char and it's ascii value
fn char_to_ascii_mapping(s: &str) -> HashMap<char, u8> {
    s.chars().map(|c| (c, c as u8)).collect()
}