use std::collections::HashMap;
use regex::Regex;
use std::fs;
use std::fmt::format;
use std::env;
#[tokio::main]
pub async fn run(){

  // this method needs to be inside main() method
  //env::set_var("RUST_BACKTRACE", "1");
    
    /*
        let exercite_url = "https://adventofcode.com/2023/day/3";
    */
    let input  = read_string_from_file("ressources/input3.txt");
    let mut part_numbers: Vec<String> = Vec::new();
    // let mut result = 0;
    // let mut games = Vec::new();
    // let mut big_power = 0;
    // Create a regex to match sequences of digits
    
    let re = Regex::new(r"\d+").unwrap();

    for line in input.lines() {
        let new_line = line.to_string();
        let numbers: Vec<String> = re.find_iter(&new_line)
                                     .map(|mat| mat.as_str().to_string())
                                     .collect();

        let mut indices: Vec<(String, Option<usize>)> = Vec::new();
        let mut result = String::new();
        println!("Numbers: {:?}", numbers);
        for item in numbers.iter() {
            let index = line.find(item);
            indices.push((item.clone(), index));
            if let Some(index) = index {
                let start_idx = if index > 0 { index - 1 } else { index };
                let lenght = if item.chars().count() < line.chars().count() { 1 } else { 0 };
                let first_char = &get_substring(line, start_idx, item.chars().count()).chars().nth(0).unwrap();
                let last_char = &get_substring(line, item.chars().count(), lenght).chars().nth(0).unwrap();
                if item == "102" {
                    println!("{}", last_char);
                }
                if check_is_symbol(*first_char) || check_is_symbol(*last_char) {
                    result += &get_substring(line, index, item.chars().count());
                }
            }
        }

        println!("Result: {}", result);
    }
}

fn check_is_symbol(c: char) -> bool {
    !c.is_digit(10) && c != '.'
}

fn get_substring(input: &str, start: usize, length: usize) -> String {
    let end_index = start + length;
    return format!("{}, ", input[start..end_index].to_string())
}

fn read_string_from_file(path: &str) -> String {
    match fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => {
            println!("file not found due to : {}", e);
            return String::new()}, // Return an empty String if there's an error
    }
}