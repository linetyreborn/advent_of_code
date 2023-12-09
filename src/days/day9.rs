use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn run() {
    let input = read_string_from_file("ressources/input9.txt");
    let (result1 , result2) = get_steps(&input);
    println!("part 1 - result : {}", result1);
    println!("part 1 - result : {}", result2);
}

fn get_steps(input: &str) -> (i128, i128) {
    let mut lines = input.lines();
    let (mut result1 , mut result2) = (0, 0);
    for line in lines {
        let mut vals = parse_vals(line);
        let mut result_2 = vals[0];
        let mut add = true;
        while !vals.iter().all(|&val| val == 0) {
            result1 += vals[vals.len()-1];
            add = !add;
            let mut new_vals = Vec::new();
            let mut i = 0;
            while i < vals.len() - 1 {
                new_vals.push(vals[i+1] - vals[i]);
                i += 1;
            }
            vals = new_vals;  
            result_2 = if add {result_2 + vals[0]} else {result_2 - vals[0]};
        }
        result2 += result_2;
    }   
    (result1, result2)
}

fn parse_vals(input: &str) -> Vec<i128> {
    let vals: Vec<i128> = input.split_whitespace().filter_map(|n| n.parse::<i128>().ok()).collect();
    return vals;
}

fn read_string_from_file(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| panic!("Unable to read file: {}", path))
}