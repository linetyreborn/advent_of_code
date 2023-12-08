use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn run() {
    let input = read_string_from_file("ressources/input8.txt");
    println!("{}", get_steps(&input));
}

fn get_steps(input: &str) -> i128 {
    let mut nodes = HashMap::new();
    let mut lines = input.lines();
    let mut steps = 0;

    if let Some(directions) = lines.next() {
        let directions: Vec<char> = directions.trim().chars().collect();

        for line in lines {
            if let Some(node) = parse_node(line) {
                nodes.insert(node.input.clone(), node);
            }
        }

        let mut current_nodes: Vec<&Node> = nodes
            .values()
            .filter(|node| node.input.ends_with("A"))
            .collect();

        let mut index = 0;

        while !current_nodes.iter().all(|node| node.input.ends_with("Z")) {
            // Debugging output
            // println!("Step: {}, Current Nodes: {:?}", steps, current_nodes);

            let current_direction = directions[index % directions.len()];

            for current_node in current_nodes.iter_mut() {
                let next_node = match current_direction {
                    'R' => nodes.get(&(*current_node).right),
                    'L' => nodes.get(&(*current_node).left),
                    _ => None,
                };

                if let Some(node) = next_node {
                    *current_node = node;
                }
            }

            steps += 1;
            index += 1;

        }
    }

    steps
}


#[derive(Debug, Clone)]
struct Node {
    input: String,
    left: String,
    right: String
}

fn parse_node(input: &str) -> Option<Node> {
    let cleaned_input = input
        .replace("=", "")
        .replace("(", "")
        .replace(")", "")
        .replace(",", "");

    let parts: Vec<&str> = cleaned_input.split_whitespace().collect();

    if parts.len() == 3 {
        return Some(Node {
            input: parts[0].to_string(),
            left: parts[1].to_string(),
            right: parts[2].to_string(),
        });
    }
    None
}

fn read_string_from_file(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| panic!("Unable to read file: {}", path))
}