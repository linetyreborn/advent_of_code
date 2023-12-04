
use std::collections::{HashSet, HashMap};
use regex::Regex;
use std::fs;
use std::fmt::format;
use std::env;
pub  fn run() {
    let input  = read_string_from_file("ressources/input4.txt");

    // Define a regular expression to match numbers
    let re = Regex::new(r"\d+").unwrap();
    // Split the input into lines and process them
    let mut result = 0;
    let mut details_input: HashMap<(i32, i32), (&str, u32)> = HashMap::new();
    let mut i: i32 = 0;
    for (idx , line) in input.lines().enumerate() {
        // let mut new_line = line.to_string();
        let mut parts: Vec<&str> = line.split(':').collect();
        let game_id= parts[0];
        let parts: Vec<&str> = parts[1].split('|').collect();

        // Parse the numbers from each part
        let array1 = parse_numbers(parts[0]);
        let array2 = parse_numbers(parts[1]);
        let matching_count = count_matching(&array1, &array2);
        result += matching_count;
        if let Some(g_id) = parse_gameid(game_id) {
            details_input.insert((g_id, 1), (line , matching_count));
            println!("{:?} {:?}",g_id,  (line , matching_count));
        }
        
    }
    println!(" result : {}", result);
    details_input = create_copies((1,1), details_input);
    println!(" result : {:?}", details_input.len());
}

fn create_copies(current_card: (i32, i32), mut details_input: HashMap<(i32, i32), (&str, u32)>) -> HashMap<(i32, i32), (&str, u32)> {
    println!(" current_card : {:?}", current_card);
    let mut details_input_result = details_input.clone();
    if let Some(details_input_current) = details_input_result.get(&current_card) {
        let matching_count = details_input_current.1 as i32;
        let position = current_card.0;
        let top: i32 = (position + matching_count).min((details_input.len()) as i32);
        let start_pos = position+1;
        for x in start_pos..top+1 {
            if let Some(highest_y) = highest_y_for_x(x, &details_input){
                details_input.insert((x, highest_y + 1), details_input_current.clone());
            }
        }

        let next_card = get_details_input_next(current_card, &details_input);
        if next_card != current_card {
            details_input_result = create_copies(next_card, details_input);
        }
    }
    details_input_result
}
    
fn get_details_input_next(current_card: (i32, i32), details_input: &HashMap<(i32, i32), (&str, u32)>) -> (i32, i32) {
    let next_horizontal = (current_card.0, current_card.1 + 1);
    let next_vertical = (current_card.0 + 1, 1);

    if details_input.contains_key(&next_horizontal) {
        next_horizontal
    } else if details_input.contains_key(&next_vertical) {
        next_vertical
    } else {
        current_card
    }
}

fn parse_numbers(s: &str) -> Vec<i32> {
    s.split_whitespace()
        .filter_map(|word| word.parse().ok())
        .collect()
}

fn count_matching(arr1: &[i32], arr2: &[i32]) -> u32 {
    let set1: HashSet<_> = arr1.iter().collect();
    let set2: HashSet<_> = arr2.iter().collect();
    set1.intersection(&set2).count() as u32
}

fn parse_gameid(input: &str) -> Option<i32> {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    if parts.len() == 2 {
        if let Ok(number) = parts[1].parse::<i32>() {
            return Some(number);
        }
    }

    None
}

fn highest_y_for_x(x: i32, map: &HashMap<(i32, i32), (&str, u32)>) -> Option<i32> {
    map.keys()
        .filter(|&&(kx, _)| kx == x)
        .map(|&(_, ky)| ky)
        .max()
}

fn read_string_from_file(path: &str) -> String {
    match fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => {
            println!("file not found due to : {}", e);
            return String::new()}, // Return an empty String if there's an error
    }
}