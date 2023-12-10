use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use lazy_static::lazy_static;

lazy_static! {
    static ref DIRECTIONS: HashMap<String, fn(((i128, i128), (i128, i128))) -> Option<(i128, i128)>> = {
        let mut map = HashMap::new();
        map.insert("-".to_string(), step_over_horizontal_dash as fn(((i128, i128), (i128, i128))) -> Option<(i128, i128)>);
        map.insert("|".to_string(), step_over_vertical_dash);
        map.insert("7".to_string(), step_over_7);
        map.insert("L".to_string(), step_over_l);
        map.insert("F".to_string(), step_over_f);
        map.insert("J".to_string(), step_over_j);
        map.insert(".".to_string(), |_| None);
        map
    };
}


pub fn run() {
    let input = read_string_from_file("ressources/input10.txt");
    get_steps(&input);

}

fn get_steps(input: &str){
    let mut starting_char: (String, (i128, i128)) = ("S".to_string(), (0,0));
    let mut char_map = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch.to_string() == starting_char.0 {
                starting_char.1 = (x as i128, y as i128);
            }
            char_map.insert((x as i128, y as i128), ch.to_string());
        }
    }
    let mut test: Vec<i128> = Vec::new();
    let mut loop_members: Vec<(i128, i128)> = Vec::new();
    if let Some(positions) = step_over_s((starting_char.1, starting_char.1)) {

        for position in positions {
            let mut length = 0;
            let mut old_char_pos:(i128, i128) = starting_char.1;
            let mut old_char = "9";
            if let Some(new_char) =  char_map.get(&position) {
                let mut new_pos :(i128, i128) = position; 
                if *new_char != "." {
                    loop_members.push(new_pos);
                    if let Some(step_over_fn) = DIRECTIONS.get(new_char) {
                        if let Some(new_current_pos) = step_over_fn((position, old_char_pos)) {
                            old_char_pos = position;
                            new_pos = new_current_pos;
                            old_char = new_char;
                            length += 1;
                        }
                    }           
                    let mut stop = false;
                    while old_char != "S" && !stop {
                        let (i,j) = old_char_pos;
                        loop_members.push(new_pos);
                        if let Some(new_char1) =  char_map.get(&new_pos) {
                            if *new_char1 != "." && *new_char1 != "S"{
                                if let Some(step_over_fn) = DIRECTIONS.get(new_char1) {
                                    if let Some(new_current_pos) = step_over_fn((new_pos, old_char_pos)) {
                                        old_char_pos = new_pos;
                                        new_pos = new_current_pos;
                                        old_char = new_char1;
                                        length += 1;
                                    }
                                }
                            } else {
                                stop = true;
                            }
                        }
                    }
                }
            }
            test.push(if length % 2 == 0 { length / 2} else { ((length-1) / 2) + 1});
            if length != 0 {
                break;
            }
        }
    }
    let max_length = test.iter().max();
    match max_length {
        Some(&max) => println!("farthest tile  is: {} away", max),
        None => println!("The vector is empty."),
    }


}

fn string_to_hashmap(text: &str) -> HashMap<(i128, i128), char> {
    let mut char_map = HashMap::new();
    for (x, line) in text.lines().enumerate() {
        for (y, ch) in line.chars().enumerate() {
            char_map.insert((x as i128, y as i128), ch);
        }
    }
    char_map
}

fn step_over_s(coords: ((i128, i128), (i128, i128))) -> Option<Vec<(i128, i128)>> {
    let ((x, y), _) = coords;
    Some(vec![(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)])
}

fn step_over_horizontal_dash(coords: ((i128, i128), (i128, i128))) -> Option<(i128, i128)> {
    let ((x, y), (i, j)) = coords;
    if x == i + 1 && y == j {
        Some((x + 1, y))
    } else if x == i - 1 && y == j {
        Some((x - 1, y))
    } else {
        None
    }
}

fn step_over_vertical_dash(coords: ((i128, i128), (i128, i128))) -> Option<(i128, i128)> {
    let ((x, y), (i, j)) = coords;
    if y == j + 1 && x == i {
        Some((x, y + 1))
    } else if y == j - 1 && x == i {
        Some((x, y - 1))
    } else {
        None
    }
}

fn step_over_7(coords: ((i128, i128), (i128, i128))) -> Option<(i128, i128)> {
    let ((x, y), (i, j)) = coords;
    if x == i + 1 && y == j {
        Some((x, y + 1))
    } else if x == i && y == j - 1 {
        Some((x - 1, y))
    } else {
        None
    }
}

fn step_over_l(coords: ((i128, i128), (i128, i128))) -> Option<(i128, i128)> {
    let ((x, y), (i, j)) = coords;
    if x == i && y == j + 1 {
        Some((x + 1, y))
    } else if x == i - 1 && y == j {
        Some((x, y - 1))
    } else {
        None
    }
}

fn step_over_f(coords: ((i128, i128), (i128, i128))) -> Option<(i128, i128)> {
    let ((x, y), (i, j)) = coords;
    if x == i - 1 && y == j {
        Some((x, y + 1))
    } else if x == i && y == j - 1 {
        Some((x + 1, y))
    } else {
        None
    }
}

fn step_over_j(coords: ((i128, i128), (i128, i128))) -> Option<(i128, i128)> {
    let ((x, y), (i, j)) = coords;
    if x == i && y == j + 1 {
        Some((x - 1, y))
    } else if x == i + 1 && y == j {
        Some((x, y - 1))
    } else {
        None
    }
}

fn read_string_from_file(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| panic!("Unable to read file: {}", path))
}