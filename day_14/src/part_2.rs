use std::include_str;
use std::collections::HashMap;

fn get_pair_rule_from_line(line: &str) -> (String, char) {
    let mut line_split: Vec<String> = line.split(" -> ").map(str::to_string).collect();
    let l = line_split.remove(0);
    let r = line_split.remove(0).chars().collect::<Vec<char>>()[0];
    (l, r)
}

fn main() {
    let mut lines = include_str!("../input.txt").lines();
    // let mut lines = include_str!("../example.txt").lines();

    let template = lines.next().unwrap().to_string();
    lines.next(); // blank line

    let pair_rule_map: HashMap<String, char> = lines.map(get_pair_rule_from_line).collect();

    let mut pair_counts: HashMap<String, u64> = HashMap::new();
    let mut elem_counts: HashMap<char, u64> = HashMap::new();

    for (i, elem) in template.chars().enumerate() {
        let elem_count = elem_counts.entry(elem).or_insert(0);
        *elem_count += 1;

        if i >= template.len()-1 {
            continue;
        }

        let pair: String = template[i..i+2].to_string();
        let pair_count = pair_counts.entry(pair).or_insert(0);
        *pair_count += 1;
    }

    for _ in 0..40 {
        let mut new_pair_counts: HashMap<String, u64> = HashMap::new();
        for (pair, count) in pair_counts.iter() {
            let new_elem = pair_rule_map[pair];
            let elem_count = elem_counts.entry(new_elem).or_insert(0);
            *elem_count += count;

            let left = pair.chars().nth(0).unwrap();
            let right = pair.chars().nth(1).unwrap();

            let new_left: String = vec![left, new_elem].iter().collect();
            let new_right: String = vec![new_elem, right].iter().collect();

            let new_left_entry = new_pair_counts.entry(new_left).or_insert(0);
            *new_left_entry = *new_left_entry + *count;

            let new_right_entry = new_pair_counts.entry(new_right).or_insert(0);
            *new_right_entry = *new_right_entry + *count;
        }
        pair_counts = new_pair_counts;
    }

    let most_common = *elem_counts.values().max().unwrap();
    let least_common = *elem_counts.values().min().unwrap();

    println!("{}", most_common - least_common);
}
