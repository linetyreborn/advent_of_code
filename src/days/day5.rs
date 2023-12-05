use std::fs;
use std::collections::HashMap;

pub fn run() {
    let input = read_string_from_file("ressources/input5.txt");
    let seeds = parse_seeds(&input);
    println!("seed: {:?}", seeds);
    if let Some(lowest_location_seed) = find_seed_with_lowest_location(&seeds) {
        println!("Seed with lowest location: {:?}", lowest_location_seed);
    } else {
        println!("No seed with a location found");
    }
}

fn parse_seeds(input: &str) -> HashMap<i64, Seed> {
    let mut input_seeds: Vec<i64> = Vec::new();
    let mut input_seeds_map: HashMap<i64, Seed> = HashMap::new();
    let mut previous_category = String::new();
    let mut next_category = String::new();

    for line in input.lines().filter(|line| !line.is_empty()) {
        if line.starts_with("seeds:") {
            input_seeds = line.split_whitespace().skip(1).filter_map(|n| n.parse().ok()).collect();
        } else {
            let parts = line.split(" ").collect::<Vec<&str>>();
            if parts.len() == 2 {
                if let Some(first_part) = parts.first() {
                    let categories = first_part.split("-to-").collect::<Vec<&str>>();
                    if categories.len() == 2 {
                        previous_category = categories[0].to_string();
                        next_category = categories[1].to_string();
                    }
                }
            } else if parts.len() == 3 {
                let dest_source_range: Vec<i64> = parts.iter().filter_map(|&n| n.parse().ok()).collect();
                if let [dest, source, range] = dest_source_range[..] {
                    for &input_seed in &input_seeds {
                        let seed = input_seeds_map.entry(input_seed).or_insert_with(|| Seed {
                            seed_number: input_seed,
                            additional_info : HashMap::new(),
                        });

                        let current_id = if previous_category != "seed" {
                            *seed.additional_info.get(&previous_category).unwrap_or(&input_seed)
                        } else {
                            input_seed
                        };

                        let new_val = if current_id >= source && current_id <= source + range - 1 {
                            current_id + (dest - source)
                        } else {
                            current_id
                        };

                        seed.additional_info.insert(next_category.clone(), new_val);
                    }
                }
            }
        }
    }

    input_seeds_map
}

#[derive(Debug, Clone)]
struct Seed {
    seed_number: i64,
    additional_info: HashMap<String, i64>,
}

fn parse_part(part: &str) -> Result<i64, std::num::ParseIntError> {
    part.parse()
}

fn find_seed_with_lowest_location(seeds: &HashMap<i64, Seed>) -> Option<&Seed> {
    seeds.values()
        .filter(|seed| seed.additional_info.contains_key("location"))
        .min_by_key(|seed| seed.additional_info.get("location").unwrap_or(&i64::MAX))
}

fn read_string_from_file(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| panic!("Unable to read file: {}", path))
}