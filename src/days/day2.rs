use std::collections::HashMap;
use std::fs;
pub async fn run(){
    
    /*
        let exercite_url = "https://adventofcode.com/2023/day/2";
    */
    let input  = read_string_from_file("ressources/input2.txt");
    
    let mut inputs: HashMap<String, i32> = HashMap::new();
    inputs.insert("red".to_string(), 12);
    inputs.insert("blue".to_string(), 14);
    inputs.insert("green".to_string(), 13);

    let mut result = 0;
    let mut games = Vec::new();
    let mut big_power = 0;
    for line in input.lines() {
        let game = line.split(":").collect::<Vec<&str>>();
        let game_id = game[0];
        let rounds = game[1].split(";").collect::<Vec<&str>>();
        let mut is_valid_game = true;
        let mut min_cubes_vect: HashMap<String, i32> = HashMap::new();
        for round in rounds {
            let mut cubes_vect: HashMap<String, i32> = HashMap::new();
            let cubes = round.split(",").collect::<Vec<&str>>();
            for cube in cubes {
                if let Some((num, color)) = parse_cube(cube) {
                    cubes_vect.entry((&color).to_string()).and_modify(|e| *e += num).or_insert(num);
                    min_cubes_vect.entry((&color).to_string()).and_modify(|e| *e = if *e < num { num } else { *e }).or_insert(num);
                }
            }
            for (k, v) in inputs.iter() {
                match cubes_vect.get(k) {
                    Some(&cube_value) => {
                        if cube_value > *v {
                            is_valid_game = false;
                            break;
                        }
                    },
                    None => {}
                }
            }
        }
        let mut power = 1;
        for (_k, v) in min_cubes_vect.iter() {
            power *= *v;
        }
        big_power += power;
        if is_valid_game {
            if let Some(id) = parse_gameid(game_id)  {
                games.push(id);
                result += id;
            }
        }
    }
    println!("result : {}", result);
    println!("big_power : {}", big_power);
}

fn parse_cube(input: &str) -> Option<(i32, &str)> {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    if parts.len() == 2 {
        if let Ok(number) = parts[0].parse::<i32>() {
            return Some((number, parts[1]));
        }
    }

    None
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

fn read_string_from_file(path: &str) -> String {
    match fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => {
            println!("file not found due to : {}", e);
            return String::new()}, // Return an empty String if there's an error
    }
}