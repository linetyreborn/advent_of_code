use std::collections::HashMap;
use regex::Regex;
use std::fs;
use std::fmt::format;
use std::env;

pub fn run() {
      // this method needs to be inside main() method
    // env::set_var("RUST_BACKTRACE", "full");
    let input_strings  = read_string_from_file("ressources/input3.txt");
    
    let input_strings = "
    ...733.......289..262.....520..................161.462..........450.........................183..................100*.......................
    ....*....................*.............707.352....*............/.....................801...@...............333..196........484.635......287*
    ....42.........131....913..............*......&..........634..................440..&...............83.....@...........404$..=....*..423.*...
    618.......272....*.........&......547.344...............#............689.589.*....150......382=................................168......433.";
    
     //*/
    let mut full_indices1 = HashMap::new();
    let mut size = 0;

    let re = Regex::new(r"\d+").unwrap();

    // let mut full_indices: Vec<(usize, Vec<usize>)> = Vec::new();
    for (idx , input_string_input) in input_strings.lines().enumerate() {
        let mut input_string = input_string_input.trim();
        let new_line = input_string.to_string();
        let numbers: Vec<String> = re.find_iter(&new_line)
                                     .map(|mat| mat.as_str().to_string())
                                     .collect();
        let mut indices = Vec::new();
        for (index, character) in input_string.char_indices() {
            if !character.is_digit(10) && character != '.' && character != '\n' && character != ' ' {
                indices.push(index);
                // println!("{:?} {:?} {:?}", index , input_string.chars().nth(index), input_string.chars().nth(index + 1));
            }
        }
        // full_indices.push((idx, indices.clone()));
        full_indices1.insert(idx,(input_string, indices, numbers));
        size += 1;
    }
    for (idx , input_string) in input_strings.lines().enumerate() {
        
    }
    let mut i: usize = 0;
    let mut valid_part_numbers= HashMap::new();
    
    while i < size {
        let mut line_i_minus_1 = (0, "", Vec::new(), Vec::new());
        let mut line_i = (0, "", Vec::new(), Vec::new());

        if i > 0 {
            if let Some((str_line, idx,numbers)) = full_indices1.get(&(i -1)) {
                line_i_minus_1 = (i-1, str_line, idx.to_vec(), numbers.to_vec());
            }
        }
        if let Some((str_line, idx , numbers)) = full_indices1.get(&(i)) {
            line_i = (i, str_line, idx.to_vec(), numbers.to_vec());

        }


         if line_i_minus_1.3.len() > 0 {
            for num in line_i_minus_1.3.clone().into_iter(){
                if let Some(num_pos) = line_i_minus_1.1.find(&num) {
                    for sym_pos in line_i.2.iter() {
                        if num_pos > 0 && &(num_pos - 1) <= sym_pos && sym_pos <= &(num_pos + num.len()) {
                            valid_part_numbers.insert((line_i_minus_1.0, num_pos), num.clone());
                        } else if num_pos == 0 && sym_pos <= &(num.len()) {
                            valid_part_numbers.insert((line_i_minus_1.0, num_pos), num.clone());
                        }
                    }
                }
            }
        }

        for sym_pos in line_i_minus_1.2.iter() {
            for num in line_i_minus_1.3.iter() {
                if let Some(num_pos) = line_i_minus_1.1.find(&*num) {
                    if num_pos > 0 && &(num_pos - 1) <= sym_pos && sym_pos <= &(num_pos + num.len()) {
                        valid_part_numbers.insert((line_i_minus_1.0, num_pos), num.clone());
                    } else if num_pos == 0 && sym_pos <= &(num.len()) {
                        valid_part_numbers.insert((line_i_minus_1.0, num_pos), num.clone());
                    }
                }
            }
            for num in &line_i.3{
                if let Some(num_pos) = line_i.1.find(&*num) {
                    if num_pos > 0 && &(num_pos - 1) <= sym_pos && sym_pos <= &(num_pos + num.len()) {
                        valid_part_numbers.insert((line_i.0, num_pos), num.clone());
                    } else if num_pos == 0 && sym_pos <= &(num.len()) {
                        valid_part_numbers.insert((line_i.0, num_pos), num.clone());
                    }
                }
            }
        }

        for sym_pos in line_i.2.iter() {
            for num in line_i.3.iter() {
                if let Some(num_pos) = line_i.1.find(&*num) {
                    if num_pos > 0 && &(num_pos - 1) <= sym_pos && sym_pos <= &(num_pos + num.len()) {
                        valid_part_numbers.insert((line_i.0, num_pos), num.clone());
                    } else if num_pos == 0 && sym_pos <= &(num.len()) {
                        valid_part_numbers.insert((line_i.0, num_pos), num.clone());
                    }
                }
            }
        }
        // println!("line_i {} : {:?}",i , line_i);
        i += 1;
    }
    // println!("valid_part_numbers : {:?}", valid_part_numbers);
    let mut somme = 0;
    for (k, v) in valid_part_numbers {
        if let Ok(calibration) = v.parse::<i32>() {
            somme += calibration;
        }
    }
    println!("somme : {:?}", somme);   

}


fn read_string_from_file(path: &str) -> String {
    match fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => {
            println!("file not found due to : {}", e);
            return String::new()}, // Return an empty String if there's an error
    }
}