use regex::Regex;
use std::fs;
#[tokio::main]
pub async fn run() {

    /*
        let exercite_url = "https://adventofcode.com/2023/day/1";
    */
    let digits = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9")
    ];
    let input  = read_string_from_file("ressources/input1.txt");
    // Define a regular expression to match numbers
    let re = Regex::new(r"\d+").unwrap();
    let mut calibration_sum: i32 = 0;
    // Split the input into lines and process them
    for line in input.lines() {
        let mut new_line = line.to_string();
        let mut positions = Vec::new();
        for (key, value) in digits.iter() {

            if let Some(pos) = line.find(key) {

                positions.push((key, value,pos));
            }
        }

        positions.sort_by_key(|&(_, _, pos)| pos);
        // Find min and max based on positions, if any
        if let Some((min_key, min_value, _)) = positions.get(0) {
            new_line = new_line.replacen(&**min_key, &min_value, 1);
        }

        if let Some((max_key, max_value, _)) = positions.last() {
            // new_line = new_line.rfind(&max_key, &max_value)
            new_line = replace_last(&new_line, max_key, max_value);
            // new_line = new_line.replace(&**max_key, &max_value);
        }
         //println!("{} : {}", line, new_line);
        // Find all occurrences of numbers in the line
        let numbers: Vec<&str> = re.find_iter(&new_line).map(|m| m.as_str()).collect();
        // Ensure that the line contains at least one valid number
        if !numbers.is_empty() {
                // Extract the first and last numbers
            let first_number = numbers.first().map(|&num| num.chars().next().unwrap_or('0')).unwrap_or('0');
            let last_number = numbers.last().map(|&num| num.chars().last().unwrap_or('0')).unwrap_or('0');

            // Convert the extracted characters to strings
            let first_digit = first_number.to_string();
            let last_digit = last_number.to_string();
            let calibration_str = format!("{}{}",first_digit,last_digit);
            if let Ok(calibration) = calibration_str.parse::<i32>() {
                calibration_sum += calibration;
            } else {
                println!("Failed to parse numbers on line: {}", line);
            }
        }
    }
    println!("Result: {}", calibration_sum);
}


fn replace_last(s: &str, pattern: &str, replacement: &str) -> String {
    if let Some(pos) = s.rfind(pattern) {
        let (start, end) = s.split_at(pos);
        format!("{}{}{}", start, replacement, &end[pattern.len()..])
    } else {
        s.to_string()
    }
}

fn read_string_from_file(path: &str) -> String {
    match fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => {
            println!("file not found due to : {}", e);
            return String::new()}, // Return an empty String if there's an error
    }
}