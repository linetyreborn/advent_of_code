use std::fs;
use std::collections::HashMap;

pub fn run() {
    let input = read_string_from_file("ressources/input4.txt");
    let cards = parse_cards(&input);
    let total = process_cards(&cards);
    println!("Total scratchcards: {}", total);
}

fn parse_cards(input: &str) -> Vec<ScratchCard> {
    input.lines().filter_map(|line| {
        let parts: Vec<&str> = line.split(" | ").collect();
        if parts.len() == 2 {
            let winning_numbers: Vec<i32> = parts[0].split_whitespace().filter_map(|n| n.parse().ok()).collect();
            let my_numbers: Vec<i32> = parts[1].split_whitespace().filter_map(|n| n.parse().ok()).collect();
            Some(ScratchCard { winning_numbers, my_numbers })
        } else {
            None
        }
    }).collect()
}

fn process_cards(cards: &[ScratchCard]) -> usize {
    let mut count = HashMap::new();
    let mut to_process = Vec::new();

    for (index, card) in cards.iter().enumerate() {
        to_process.push((index, card));
    }

    while let Some((index, card)) = to_process.pop() {
        let matches = card.count_matches();
        *count.entry(index).or_insert(0) += 1;

        for i in 1..=matches {
            if let Some(next_card) = cards.get(index + i) {
                to_process.push((index + i, next_card));
            }
        }
    }

    count.values().sum()
}

struct ScratchCard {
    winning_numbers: Vec<i32>,
    my_numbers: Vec<i32>,
}

impl ScratchCard {
    fn count_matches(&self) -> usize {
        self.my_numbers.iter().filter(|n| self.winning_numbers.contains(n)).count()
    }
}

fn read_string_from_file(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| panic!("Unable to read file: {}", path))
}