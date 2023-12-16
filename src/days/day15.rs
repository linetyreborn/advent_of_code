use std::fs;
use std::collections::HashMap;
use std::cmp;

pub fn run() {
    let input = read_string_from_file("ressources/input14.txt");
    calculate_weight(&input);

}

fn calculate_weight(input: &str) {
    let mut result = 0;
    let (rows, columns)= parse_string(input);
    // let mut new_columns = Vec::new();
    for column in columns {
        let mut new_column = column.clone();
        let mut parts = split_vec_by_char(column, '#');

        for (key, part) in parts.iter_mut(){
            let length = part.len();
            part.retain(|&c| c != '.');
            let additional_length = length.saturating_sub(part.len());
            part.extend(std::iter::repeat('.').take(additional_length));
            for i in 0..part.len() {
                new_column[i+ key] = part[i];
            }
        }
        for (idx, c) in new_column.iter().enumerate() {
            if *c == 'O' {
                result += new_column.len() - idx;
            }
        }
    }
    println!("{}", result);
}

fn parse_string(input: &str) -> (Vec<Vec<char>>, Vec<Vec<char>>) {
    let rows: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let max_col_length = rows.iter().map(|row| row.len()).max().unwrap_or(0);
    let mut columns: Vec<Vec<char>> = vec![Vec::new(); max_col_length];

    for row in &rows {
        for (i, &c) in row.iter().enumerate() {
            columns[i].push(c);
        }
    }

    (rows, columns)
}


fn split_vec_by_char(vec: Vec<char>, delimiter: char) -> HashMap<usize, Vec<char>>  {
    let mut result: HashMap<usize, Vec<char>> = HashMap::new();
    let mut current_vec: Vec<char> = Vec::new();
    let mut start_index = 0;

    for (i, c) in vec.into_iter().enumerate() {
        if c == delimiter {
            if !current_vec.is_empty() {
                result.insert(start_index,current_vec);
                current_vec = Vec::new();
            }
            start_index = i + 1; // next segment starts after the delimiter
        } else {
            current_vec.push(c);
        }
    }

    // Push the last vector if it's not empty
    if !current_vec.is_empty() {
        result.insert(start_index,current_vec);
    }

    result
}



fn read_string_from_file(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| panic!("Unable to read file: {}", path))
}