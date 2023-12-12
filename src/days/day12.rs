use std::fs;
use std::collections::HashMap;
use regex::Regex;

pub fn run() {
    let input = read_string_from_file("ressources/input12.txt");
    println!("{}", get_steps(&input));

}

fn get_steps(input: &str) -> i128 {
    let mut solution = 0;
    let folding_factor = 1;

    for line in input.lines() {
        let parts: Vec<&str> = line.trim().split_whitespace().collect();

        if let [first_part, second_part] = parts.as_slice() {
            let numbers: Vec<i128> = second_part
                .split(',')
                .filter_map(|s| s.parse().ok())
                .collect();
            let repeated_string = generate_repeated_string(first_part, folding_factor);
            let result = generate_strings_with_permutations(&repeated_string, &numbers);
            solution += result;
        }
    }

    solution
}

fn generate_strings_with_permutations(input: &str, numbers: &Vec<i128>) -> i128 {
    generate_strings_recursive(input, 0, String::new(), numbers)
}

fn generate_strings_recursive(input: &str, index: usize, current: String, numbers: &Vec<i128>) -> i128 {
    if index == input.len() {
        if extract_lengths_of_repeated_hashes(&current) == *numbers {
            return 1;
        }
        return 0;
    }

    let c = input.chars().nth(index).unwrap();
    if c == '?' {
        let mut result = 0;
        result += generate_strings_recursive(input, index + 1, current.clone() + ".", numbers);
        result += generate_strings_recursive(input, index + 1, current.clone() + "#", numbers);
        result
    } else {
        generate_strings_recursive(input, index + 1, current.clone() + &c.to_string(), numbers)
    }
}

fn extract_lengths_of_repeated_hashes(input: &str) -> Vec<i128> {
    // Split the input string by '.' to get a vector of segments
    let segments: Vec<&str> = input.split('.').collect();

    // Filter segments to keep only non-empty segments (containing '#')
    let non_empty_segments: Vec<&str> = segments.iter().filter(|&&seg| !seg.is_empty()).copied().collect();

    // Extract the lengths of the repeated '#' characters in each non-empty segment
    let lengths: Vec<i128> = non_empty_segments.iter().map(|seg| {
        let mut repeated_hash_length = 0;
        let mut prev_char: Option<char> = None;

        for c in seg.chars() {
            match prev_char {
                Some(ch) if ch == '#' && ch == c => repeated_hash_length += 1,
                _ => {
                    prev_char = Some(c);
                    if c == '#' {
                        repeated_hash_length = 1;
                    } else {
                        repeated_hash_length = 0;
                    }
                }
            }
        }

        repeated_hash_length
    }).collect();

    lengths
}

fn repeat_vec(original: &Vec<i128>, times: usize) -> Vec<i128> {
    original.iter().cloned().cycle().take(original.len() * times).collect()
}

fn replace_repeated_dots(input: &str) -> String {
    let re = Regex::new(r"\.{2,}").unwrap();
    let replaced = re.replace_all(input, ".");
    replaced.to_string()
}

fn generate_repeated_string(first_part: &str, folding_factor: i8) -> String {
    let repeated_part = "?".to_string() + first_part;
    let repeated_parts = repeated_part.repeat((folding_factor as usize) - 1);

    return first_part.to_string() + &repeated_parts;
}
fn read_string_from_file(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| panic!("Unable to read file: {}", path))
}