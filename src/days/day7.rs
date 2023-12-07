use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use lazy_static::lazy_static;

lazy_static! {
    static ref HAND_STRENGTHS: HashMap<CardType, i32> = {
        let mut map = HashMap::new();
        map.insert(CardType::FiveOfKind, 7);
        map.insert(CardType::FourOfKind, 6);
        map.insert(CardType::FullHouse, 5);
        map.insert(CardType::ThreeOfKind, 4);
        map.insert(CardType::TwoPair, 3);
        map.insert(CardType::OnePair, 2);
        map.insert(CardType::HighCard, 1);
        map
    };

    static ref CARD_STRENGTHS: HashMap<String, i32> = {
        let mut map = HashMap::new();
        map.insert("A".to_string(), 13);
        map.insert("K".to_string(), 12);
        map.insert("Q".to_string(), 11);
        // part 1
        // map.insert("J".to_string(), 10);
        map.insert("T".to_string(), 9);
        map.insert("9".to_string(), 8);
        map.insert("8".to_string(), 7);
        map.insert("7".to_string(), 6);
        map.insert("6".to_string(), 5);
        map.insert("5".to_string(), 4);
        map.insert("4".to_string(), 3);
        map.insert("3".to_string(), 2);
        map.insert("2".to_string(), 1);
        //part 2
        map.insert("J".to_string(), 0);
        map
    };
}


pub fn run() {
    let input = read_string_from_file("ressources/input7.txt");
    parse_cards(&input);

}

fn parse_cards(input: &str){
    let mut hands= Vec::new();
    for line in input.lines() {
        if let Some(hand) = parse_hand(line) {
            hands.push(hand);
        }
    }
    hands.sort();
    let mut result = 0;
    for i in 0..hands.len() {
        result += hands[i].bid * ((i+1) as i32);
    }
    println!("{}", result);

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum CardType {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: String,
    hand_type: CardType,
    bid: i32
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_hand_type = self.hand_type.clone();
        let other_hand_type = other.hand_type.clone();

        // Compare the hand types using HAND_STRENGTHS
        let hand_type_cmp = HAND_STRENGTHS
            .get(&self_hand_type)
            .unwrap_or(&0)
            .cmp(&HAND_STRENGTHS.get(&other_hand_type).unwrap_or(&0));

        if hand_type_cmp != std::cmp::Ordering::Equal {
            return hand_type_cmp;
        }

        // If the hand types are the same, compare the individual cards based on CARD_STRENGTHS
        let self_cards = self.cards.chars().collect::<Vec<char>>();
        let other_cards = other.cards.chars().collect::<Vec<char>>();

        for (self_card, other_card) in self_cards.iter().zip(other_cards.iter()) {
            let self_card_strength = CARD_STRENGTHS
                .get(&self_card.to_string())
                .unwrap_or(&0);
            let other_card_strength = CARD_STRENGTHS
                .get(&other_card.to_string())
                .unwrap_or(&0);

            match self_card_strength.cmp(&other_card_strength) {
                std::cmp::Ordering::Equal => continue,
                result => return result,
            }
        }

        // If all cards are equal, return Equal
        std::cmp::Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn get_hand_type(hand : &str) -> CardType {
    let repeats = find_repeated_chars(hand);
    if repeats.len() == 1 && repeats.values().any(|&value| value == 5) {
        return CardType::FiveOfKind;
    } 
    if repeats.len() == 1 && repeats.values().any(|&value| value == 4) {
        if hand.replace("J", "").len() == 4 {
            return CardType::FiveOfKind;
        }
        return CardType::FourOfKind;
    } 
    if repeats.len() == 1 && repeats.values().any(|&value| value == 3) {
        if hand.replace("J", "").len() == 4 {
            return CardType::FourOfKind;
        }
        return CardType::ThreeOfKind;
    } 
    if repeats.len() == 2 && repeats.values().any(|&value| value == 3) {
        if hand.replace("J", "").len() == 3 || hand.replace("J", "").len() == 2 {
            return CardType::FiveOfKind;
        }
        return CardType::FullHouse;
    } 
    if repeats.len() == 2 {
        if hand.replace("J", "").len() == 3 {
            return CardType::FourOfKind;
        }
        if hand.replace("J", "").len() == 4 {
            return CardType::FullHouse;
        }
        return CardType::TwoPair;
    }
    if repeats.len() == 1 {
        if hand.replace("J", "").len() == 4 || hand.replace("J", "").len() == 3 {
            return CardType::ThreeOfKind;
        }
        return CardType::OnePair;
    }
    if hand.replace("J", "").len() == 4 {
        return CardType::OnePair;
    }
    return CardType::HighCard;
}

fn find_repeated_chars(input_string: &str) -> HashMap<char, usize> {
    let mut char_count = HashMap::new();

    for c in input_string.chars() {
        *char_count.entry(c).or_insert(0) += 1;
    }

    char_count.retain(|_, count| *count > 1);
    char_count
}

fn parse_hand(input: &str) -> Option<Hand> {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    if parts.len() == 2 {
        if let Ok(number) = parts[1].parse::<i32>() {
            return Some(Hand{cards: parts[0].to_string() ,hand_type:get_hand_type(parts[0]) , bid: number});
        }
    }
    None
}

fn read_string_from_file(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| panic!("Unable to read file: {}", path))
}