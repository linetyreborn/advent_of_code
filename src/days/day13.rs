use std::fs;
use std::collections::HashMap;
use std::cmp;

pub fn run() {
    let input = read_string_from_file("ressources/input13.txt");
    parse_pattern(&input);

}

fn parse_pattern(input: &str) {
    let lines: Vec<&str> = input.lines().collect();
    let mut patterns: Vec<(Vec<Vec<char>>, Vec<Vec<char>>)> = Vec::new();
    patterns.push((Vec::new(), Vec::new()));
    let mut i = 0;

    for (y, line) in lines.iter().enumerate() {
        if line.is_empty() {
            i += 1;
            patterns.push((Vec::new(), Vec::new()));
            continue;
        }

        let row: Vec<char> = line.chars().collect();
        patterns[i].0.push(row);

        for (x, c) in line.chars().enumerate() {
            if x >= patterns[i].1.len() {
                patterns[i].1.push(Vec::new());
            }
            patterns[i].1[x].push(c);
        }
    }
    println!("there is {} patterns", patterns.len());
    let mut result = 0;
    for (index, pattern) in patterns.iter().enumerate() {

        let mut max_vertical_reflexion = (0, false);
        let mut max_horizontal_reflexion = (0, false);
        for (x, column) in pattern.1.iter().enumerate() {
            if x < pattern.1.len() - 2 && (column == &pattern.1[x+2]) {
                let mut vertical_reflexion = (x+1, true);
                for i in 0..cmp::min(x, pattern.1.len() -(x+2)) {
                    if !are_equal(pattern.1[x-i].clone(),pattern.1[x+2+i].clone()) {
                        vertical_reflexion.1 = false;
                    }
                }
                if vertical_reflexion.1 {
                    if max_vertical_reflexion.0 < vertical_reflexion.0 {
                        max_vertical_reflexion = vertical_reflexion;
                    }
                    println!("{} : vertical_reflexion {:?}" ,index, vertical_reflexion);
                }
            }
            if x < pattern.1.len() - 1 && (column == &pattern.1[x+1]) {
                let mut vertical_reflexion = (x+1, true);
                for i in 0..cmp::min(x, pattern.1.len() -(x+1)) {
                    if !are_equal(pattern.1[x-i].clone(),pattern.1[x+1+i].clone()) {
                        vertical_reflexion.1 = false;
                    }
                }
                if vertical_reflexion.1 {
                    if max_vertical_reflexion.0 < vertical_reflexion.0 {
                        max_vertical_reflexion = vertical_reflexion;
                    }
                    // println!("{} : vertical_reflexion {:?}" ,index, vertical_reflexion);
                }
            }
        }
        for (x, row) in pattern.0.iter().enumerate() {
            if x < pattern.0.len() - 2 && (row == &pattern.0[x+2]) {
                let mut horizontal_reflexion = (x+1, true);
                for i in 0..cmp::min(x, pattern.0.len() -(x+2)) {
                    if !are_equal(pattern.0[x-i].clone(), pattern.0[x+2+i].clone()) {
                        horizontal_reflexion.1 = false;
                    }
                }
                if horizontal_reflexion.1 {
                    if max_horizontal_reflexion.0 < horizontal_reflexion.0 {
                        max_horizontal_reflexion = horizontal_reflexion;
                    }
                    println!("{} : horizontal_reflexion {:?}" ,index, horizontal_reflexion);
                }
            }
            if x < pattern.0.len() - 1 && (row == &pattern.0[x+1]) {
                let mut horizontal_reflexion = (x+1, true);
                for i in 0..cmp::min(x, pattern.0.len() -(x+1)) {
                    if !are_equal(pattern.0[x-i].clone() ,pattern.0[x+1+i].clone()) {
                        horizontal_reflexion.1 = false;
                    }
                }
                if horizontal_reflexion.1 {
                    if max_horizontal_reflexion.0 < horizontal_reflexion.0 {
                        max_horizontal_reflexion = horizontal_reflexion;
                    }
                    // println!("{} : horizontal_reflexion {:?}" ,index, horizontal_reflexion);
                }
            }
        }

        if max_vertical_reflexion.1 && max_horizontal_reflexion.1 {
            if max_horizontal_reflexion.0 > max_vertical_reflexion.0 {
                result += max_horizontal_reflexion.0 * 100;
            } else {
                result += max_vertical_reflexion.0;
            }
        } else if max_horizontal_reflexion.1 {
                result += max_horizontal_reflexion.0 * 100;
        }
        else if max_vertical_reflexion.1 {
                result += max_vertical_reflexion.0;
        }
    }
    println!("result: {:?}", result);
}

fn are_equal(vec1: Vec<char>, vec2: Vec<char>) -> bool {
    // part1
    // return vec1 == vec2;
    // part2
    return has_zero_or_one_difference(vec1, vec2);
}

fn has_zero_or_one_difference(vec1: Vec<char>, vec2: Vec<char>) -> bool {
    let mut count = 0;

    for (item1, item2) in vec1.iter().zip(vec2.iter()) {
        if item1 != item2 {
            count += 1;
            if count > 1 {
                return false;
            }
        }
    }

    true
}
fn read_string_from_file(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| panic!("Unable to read file: {}", path))
}