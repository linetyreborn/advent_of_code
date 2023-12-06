use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn run() {
    let input = read_string_from_file("ressources/input6.txt");
    println!(" races : {:?}", parse_races(&input));

}


fn parse_races(input: &str) -> i64 {
    let mut lines = input.lines();

    let times: Vec<i64> = match lines.next() {
        // first part of the problem
        // Some(line) => line.split_whitespace()
        //                   .filter_map(|n| n.parse().ok())
        //                   .collect(),

        // second part of the problem
        Some(line) => vec![convert_time_string_to_i64(line)],
        None => Vec::new(),
    };

    let record_distances: Vec<i64> = match lines.next() {
        // first part of the problem
        // Some(line) => line.split_whitespace()
        //                   .filter_map(|n| n.parse().ok())
        //                   .collect(),

        // second part of the problem
        Some(line) => vec![convert_time_string_to_i64(line)],
        None => Vec::new(),
    };
    let mut result = 1;
    let options:Vec<i64> = times.into_iter().zip(record_distances.into_iter())
         .map(|(time, record_distance)| Race { time, record_distance,winning_options: Default::default()  })
         .map(|race:Race| calculate_winning_options(race))
         .collect();

    for option in options {
        result *= option;
    }
    return result;
}

fn calculate_winning_options(race: Race) -> i64 {
    let mut winning_options = 0;
    for pressing_time in 0..= race.time {
        winning_options +=  if pressing_time * (race.time - pressing_time) > race.record_distance {1} else {0};
    }
    winning_options
}

#[derive(Debug, Clone)]
struct Race {
    time: i64,
    record_distance: i64,
    winning_options: i64,
}

fn convert_time_string_to_i64(input: &str) -> i64 {
    let numbers: String = input.chars()
        .filter(|c| c.is_digit(10))
        .collect();

    numbers.parse::<i64>().unwrap_or(0)
}

fn read_string_from_file(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| panic!("Unable to read file: {}", path))
}