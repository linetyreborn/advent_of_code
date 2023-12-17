use std::fs;
use std::collections::HashMap;
use std::cmp;
use lazy_static::lazy_static;

lazy_static! {
    static ref DIRECTIONS: HashMap<String, fn(((i128, i128), (i128, i128))) -> Option<Vec<(i128, i128)>>> = {
        let mut map = HashMap::new();
        map.insert("-".to_string(), step_over_horizontal_dash as fn(((i128, i128), (i128, i128))) -> Option<Vec<(i128, i128)>>);
        map.insert("|".to_string(), step_over_vertical_dash);
        map.insert("/".to_string(), step_over_7);
        map.insert("\\".to_string(), step_over_l);
        map
    };
}

pub fn run() {
    let input = read_string_from_file("ressources/input15.txt");
    compute_energy(&input);

}

fn compute_energy(input: &str) {
    let mut starting_position: (i128, i128) = (-1,0);
    let mut char_map: HashMap<(i128, i128), String> = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            char_map.insert((x as i128, y as i128), ch.to_string());
        }
    }

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

fn get_steps(starting_position: (i128, i128), map :HashMap<(i128, i128), String>) -> Vec<(i128, i128)> {
    let mut steps: Vec<(i128, i128)> = Vec::new();
    let(x , y) = starting_position;
    let mut current_positions = vec![(x + 1, y)];
    let mut current_chars: Vec<String> = current_positions.into_iter().map(|current_position| map.get(&current_position).unwrap().to_string()).collect::<Vec<String>>();
    let mut old_positions = vec![starting_position];
    let mut old_char = "".to_string();

    let mut stops:Vec<bool> = Vec::new();
    while stops.is_empty() || !stops.iter().all(|stop: &bool| stop == &true) {
        for current_char in current_chars {
            let mut stop = false;
            while !stop {
                if let Some(go_to_new_position) = DIRECTIONS.get(&current_char) {
                    if let Some(new_positions) = new_positions((current_position, old_position)) {
                        for new_position in new_positions {
                            steps.push(new_position);
                            old_position = current_position;
                            current_position = new_position;
                            old_char = current_char;
                            current_char = map.get(&current_position).unwrap().to_string();
                        }
                    } else {
                        stop = true;
                    }
                } else {
                    stop = true;
                }
            }
        }
    }
    steps

}
fn parse_string(input: &str) -> Vec<&str> {
    input.trim().split(",").collect()
}

fn read_string_from_file(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| panic!("Unable to read file: {}", path))
}

fn step_over_horizontal_dash(coords: ((i128, i128), (i128, i128))) -> Option<Vec<(i128, i128)>> {
    let ((x, y), (i, j)) = coords;
    if (y == j + 1  || y == j-1) && x == y {
        Some(vec![(x -1, y), (x + 1, y)])
    } else if x == i - 1 && y == j {
        Some(vec![(x - 1, y)])
    } else if x == i + 1 && y == j + 1 {
        Some(vec![(x + 1  , y)])
    } else {
        None
    }
}

fn step_over_vertical_dash(coords: ((i128, i128), (i128, i128))) -> Option<Vec<(i128, i128)>> {
    let ((x, y), (i, j)) = coords;
    if (x == i + 1  || x == i-1) && y == j {
        Some(vec![(x, y - 1), (x, y + 1)])
    } else if x == i && y == j - 1 {
        Some(vec![(x, y - 1)])
    } else if x == i && y == j + 1 {
        Some(vec![(x , y + 1)])
    } else {
        None
    }
}

fn step_over_7(coords: ((i128, i128), (i128, i128))) -> Option<Vec<(i128, i128)>> {
    // x, y coordinates of lens and i, j coordinates input beam
    let ((x, y), (i, j)) = coords;
    if x == i + 1 && y == j {
        Some(vec![(x, y - 1)])
    } else if x == i - 1 && y == j {
        Some(vec![(x, y + 1)])
    } else if x == i && y == j - 1 {
        Some(vec![(x + 1, y)])
    } else if x == i && y == j + 1 {
        Some(vec![(x - 1, y)])
    } else {
        None
    }
}

fn step_over_l(coords: ((i128, i128), (i128, i128))) -> Option<Vec<(i128, i128)>> {
    let ((x, y), (i, j)) = coords;
    if x == i + 1 && y == j {
        Some(vec![(x, y + 1)])
    } else if x == i - 1 && y == j {
        Some(vec![(x, y - 1)])
    } else if x == i && y == j - 1 {
        Some(vec![(x - 1, y)])
    } else if x == i && y == j + 1 {
        Some(vec![(x + 1, y)])
    } else {
        None
    }
}

// function that returns hashmap of char and it's ascii value
fn char_to_ascii_mapping(s: &str) -> HashMap<char, u8> {
    s.chars().map(|c| (c, c as u8)).collect()
}