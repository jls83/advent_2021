use std::include_str;
use std::collections::{HashMap, HashSet};

#[derive(Default)]
struct Display {
    lights: HashMap<u32, bool>,
}

impl Display {
    fn possible_pins(&self) -> HashMap<u32, HashSet<u32>> {
        let mut res: HashMap<u32, HashSet<u32>> = HashMap::new();

        let possible_pins = match self.lights.keys().count() {
            2 => vec![2, 5],
            3 => vec![0, 2, 5],
            4 => vec![1, 2, 3, 5],
            5 => vec![0, 1, 2, 3, 4, 5, 6],
            6 => vec![0, 1, 2, 3, 4, 5, 6],
            7 => vec![0, 1, 2, 3, 4, 5, 6],
            _ => vec![],
        }.into_iter().collect::<HashSet<u32>>();

        for pin in self.lights.keys() {
            let val = res.entry(*pin).or_insert(HashSet::new());
            val.extend(possible_pins.clone());
        }

        res
    }
}

#[derive(Default)]
struct Configuration {
    patterns: Vec<Display>,
    output: Vec<Display>,
    translation: HashMap::<u32, u32>,
}

impl Configuration {
    fn foo(&self) -> HashMap<u32, HashSet<u32>> {
        let mut res: HashMap<u32, HashSet<u32>> = HashMap::new();
        for pattern in &self.patterns {
            for (pin, possible_pins) in pattern.possible_pins() {
                let val = res.entry(pin).or_insert(HashSet::new());
                val.extend(possible_pins.clone());
            }
        }

        res
    }
}

fn parse_pattern(pattern_str: &str) -> Display {
    let lights: HashMap<u32, bool> = pattern_str.chars()
        .map(|c| ((c as u32) - 97, true))
        .collect();
    Display { lights }
}

fn parse_line(line: &str) -> Result<Configuration, &'static str> {
    let foo: Vec<Vec<&str>> = line.split(" | ")
        .map(|segment: &str| {
            segment.split(" ").collect()
        })
        .collect();

    match &foo[..] {
        [pattern_strs, output_strs] => {
            let patterns: Vec<Display> = pattern_strs.iter()
                .map(|p| parse_pattern(p))
                .collect();
            let output: Vec<Display> = output_strs.iter()
                .map(|p| parse_pattern(p))
                .collect();
            Ok(Configuration { patterns, output, translation: HashMap::new() }
        )},
        _ => Err("Whoops"),
    }

}

fn main() {
    let configs = include_str!("../example.txt")
        .lines()
        .map(parse_line)
        .map(Result::unwrap);

    for config in configs {
        println!("{:?}", config.foo());
    }
}
