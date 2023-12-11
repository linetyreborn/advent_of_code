use std::fs;
use std::collections::HashMap;


pub fn run() {
    let input = read_string_from_file("ressources/input11.txt");
    get_steps(&input);

}

fn get_steps(input: &str) {
    let lines: Vec<&str> = input.lines().collect();

    let mut hash_positions: Vec<(i32, i32)> = Vec::new();
    let mut empty_vertical_lines: Vec<bool> = vec![true; lines[0].len()];
    let mut empty_horizontal_lines: Vec<bool> = vec![false; lines.len()];
    for (y, line) in lines.iter().enumerate() {
        let mut empty_row = true;
        for (x, c) in line.chars().enumerate() {
            if c =='#' {
                    hash_positions.push((x as i32, y as i32));
                    empty_row = false;
                    empty_vertical_lines[x] = false;
            }
        }
        if empty_row {
            empty_horizontal_lines[y] = true;
        }
    }
    
    let empty_vertical_lines_pos: Vec<(i32, i32)> = empty_vertical_lines
        .into_iter()
        .enumerate()
        .filter(|&(_, is_empty)| is_empty)
        .map(|(x, _)| (x as i32, 0))
        .collect();
    
    let empty_horizontal_lines_pos: Vec<(i32, i32)> = empty_horizontal_lines
        .into_iter()
        .enumerate()
        .filter(|&(_, is_empty)| is_empty)
        .map(|(y, _)| (0, y as i32))
        .collect();
    let mut result1 = 0;
    let mut result2 = 0;
    for i in 0..hash_positions.len() {
        for j in i + 1..hash_positions.len() {
            let movements = get_movements_between_two_galaxies(hash_positions[i], hash_positions[j]);
            let distance = movements.len() as i32;
            let new_distances = calculate_distance_after_empty_rows_and_columns_duplication(
                distance,
                (hash_positions[i], hash_positions[j]),
                (&empty_vertical_lines_pos, &empty_horizontal_lines_pos),
            );
            result1 += new_distances.0;
            result2 += new_distances.1;
        }
    }
    println!("result1 {}", result1);
    println!("result2 {}", result2);
}

fn get_movements_between_two_galaxies(galaxy1_pos: (i32, i32), galaxy2_pos: (i32, i32)) -> Vec<Movement> {
    let (dx, dy) = (galaxy1_pos.0 - galaxy2_pos.0, galaxy1_pos.1 - galaxy2_pos.1);
    let mut movements = Vec::new();

    for _ in 0..dx.abs() {
        movements.push(if dx > 0 { Movement::Right } else { Movement::Left });
    }

    for _ in 0..dy.abs() {
        movements.push(if dy > 0 { Movement::Up } else { Movement::Down });
    }

    movements
}

#[derive(Debug, Clone)]
enum Movement {
    Up,
    Down,
    Left,
    Right,
}

fn calculate_distance_after_empty_rows_and_columns_duplication(
    distance: i32,
    (galaxy1_pos, galaxy2_pos): ((i32, i32), (i32, i32)),
    (empty_vertical_lines_pos, empty_horizontal_lines_pos): (
        &Vec<(i32, i32)>,
        &Vec<(i32, i32)>,
    ),
) -> (i128, i128) {
    let expansion_factor1: i128 = 2;
    let expansion_factor2: i128 = 1000000;
    let additional_horizontal_distance = empty_horizontal_lines_pos
    .iter()
    .filter(|&&(_, y)| (galaxy1_pos.1 < y && y < galaxy2_pos.1) || (galaxy2_pos.1 < y && y < galaxy1_pos.1))
    .count() as i128;
    let additional_vertical_distance = empty_vertical_lines_pos
            .iter()
            .filter(|&&(x, _)| (galaxy1_pos.0 < x && x < galaxy2_pos.0) || (galaxy2_pos.0 < x && x < galaxy1_pos.0))
            .count() as i128;
    let total_additional_distance = additional_horizontal_distance + additional_vertical_distance;
    
    return ((distance as i128) + (expansion_factor1 -1) * total_additional_distance , 
        (distance as i128) + (expansion_factor2 -1) * total_additional_distance);
}


fn read_string_from_file(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| panic!("Unable to read file: {}", path))
}