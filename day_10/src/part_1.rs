use std::include_str;
use std::collections::{HashMap, VecDeque};

const OPEN_PAREN: char = '(';
const OPEN_CURLY: char = '{';
const OPEN_SQUARE: char = '[';
const OPEN_ANGLE: char = '<';
const CLOSED_PAREN: char = ')';
const CLOSED_CURLY: char = '}';
const CLOSED_SQUARE: char = ']';
const CLOSED_ANGLE: char = '>';

fn check_pair(open_char: char, closed_char: char) -> bool {
    match open_char {
        OPEN_PAREN => CLOSED_PAREN == closed_char,
        OPEN_CURLY => CLOSED_CURLY == closed_char,
        OPEN_SQUARE => CLOSED_SQUARE == closed_char,
        OPEN_ANGLE => CLOSED_ANGLE == closed_char,
        _ => false,
    }
}

fn check_line(line: &str) -> Option<char> {
    let mut stack: VecDeque<char> = VecDeque::new();
    for c in line.chars() {
        if c == OPEN_PAREN || c == OPEN_CURLY || c == OPEN_SQUARE || c == OPEN_ANGLE {
            stack.push_back(c);
            continue;
        }
        if !check_pair(stack.pop_back().unwrap(), c) {
            return Some(c);
        }
    }

    // NOTE: If we've gotten here, we haven't had any invalid characters, but we don't necessarily
    // have a "good" line.
    None
}

fn main() {
    // let lines = include_str!("../example.txt")
    let lines = include_str!("../input.txt")
        .lines();

    let mut score_map: HashMap<char, u32> = HashMap::new();
    score_map.insert(CLOSED_PAREN, 3);
    score_map.insert(CLOSED_CURLY, 1197);
    score_map.insert(CLOSED_SQUARE, 57);
    score_map.insert(CLOSED_ANGLE, 25137);

    let score: u32 = lines.map(check_line)
        .filter_map(|x| {
            match x {
                Some(c) => Some(score_map[&c]),
                None => None,
            }
        })
        .sum();

    println!("{}", score);
}
