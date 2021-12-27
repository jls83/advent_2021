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

fn get_closing_char(open_char: char) -> Result<char, &'static str> {
    match open_char {
        OPEN_PAREN => Ok(CLOSED_PAREN),
        OPEN_CURLY => Ok(CLOSED_CURLY),
        OPEN_SQUARE => Ok(CLOSED_SQUARE),
        OPEN_ANGLE => Ok(CLOSED_ANGLE),
        _ => Err("No match found"),
    }
}

fn check_pair(open_char: char, closed_char: char) -> bool {
    get_closing_char(open_char).unwrap() == closed_char
}

fn complete_line(line: &str) -> Result<Vec<char>, &str> {
    let mut stack: VecDeque<char> = VecDeque::new();
    for c in line.chars() {
        if c == OPEN_PAREN || c == OPEN_CURLY || c == OPEN_SQUARE || c == OPEN_ANGLE {
            stack.push_back(c);
            continue;
        }
        if !check_pair(stack.pop_back().unwrap(), c) {
            return Err("Invalid");
        }
    }

    let res: Vec<char> = stack.iter().rev()
        .filter_map(|c| {
            Some(get_closing_char(*c).unwrap())
        })
        .collect();


    Ok(res)
}

fn score_completion(completion: &Vec<char>) -> u64 {
    let mut score_map: HashMap<char, u64> = HashMap::new();
    score_map.insert(CLOSED_PAREN, 1);
    score_map.insert(CLOSED_SQUARE, 2);
    score_map.insert(CLOSED_CURLY, 3);
    score_map.insert(CLOSED_ANGLE, 4);

    completion.iter().fold(0, |a, c| a * 5 + score_map[c])
}

fn main() {
    // let lines = include_str!("../example.txt")
    let lines = include_str!("../input.txt")
        .lines();

    let mut scores: Vec<u64> = lines.map(complete_line)
        .filter_map(|x| {
            match x {
                Ok(l) => Some(l),
                Err(_) => None,
            }
        })
        .map(|pj| score_completion(&pj))
        .collect();

    scores.sort();

    println!("{}", scores[scores.len() / 2]);

}
