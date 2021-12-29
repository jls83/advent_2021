use std::include_str;
use std::collections::HashMap;

fn get_pair_rule_from_line(line: &str) -> (String, String) {
    let mut line_split: Vec<String> = line.split(" -> ").map(str::to_string).collect();
    let l = line_split.remove(0);
    let r = line_split.remove(0);
    (l, r)
}

fn foo<'a>(template: String, pair_rule_map: &HashMap<String, String>) -> String {
    let mut first = template[..template.len()-1].chars();
    let mut last = template[1..template.len()].chars();

    let blah = template.len() - 1;

    let mut new_template_parts: Vec<String> = Vec::new();

    for _ in 0..blah {
        new_template_parts.pop();
        let l = first.next().unwrap();
        let r = last.next().unwrap();
        let pair = vec![l, r].iter().collect::<String>();
        let to_insert = &pair_rule_map[&pair];

        new_template_parts.append(&mut vec![l.to_string(), to_insert.to_string(), r.to_string()]);
    }

    new_template_parts.join("")
}

fn letter_counter(template: String) -> ((char, u32), (char, u32)) {
    let mut counter: HashMap<char, u32> = HashMap::new();

    for char in template.chars() {
        let val = counter.entry(char).or_insert(0);
        *val += 1
    }

    // get the most & least common
    let mut most_common: (char, u32) = ('{', u32::MIN);
    let mut least_common: (char, u32) = ('}', u32::MAX);

    for (c, v) in counter.iter() {
        if v > &most_common.1 {
            most_common = (*c, *v);
        }
        if v < &least_common.1 {
            least_common = (*c, *v);
        }
    }

    (most_common, least_common)
}

fn main() {
    let mut lines = include_str!("../input.txt").lines();
    // let mut lines = include_str!("../example.txt").lines();

    let mut template = lines.next().unwrap().to_string();
    lines.next(); // blank line

    let pair_rules = lines.map(get_pair_rule_from_line);
    let pair_rule_map: HashMap<String, String> = HashMap::from_iter(pair_rules);

    for _ in 0..10 {
        template = foo(template, &pair_rule_map);
    }

    let (most_common, least_common) = letter_counter(template);

    println!("{}", most_common.1 - least_common.1);
}
