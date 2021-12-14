use std::include_str;
use std::collections::HashMap;

#[derive(Default)]
struct Display {
    lights: HashMap<u32, bool>,
}

#[derive(Default)]
struct Configuration {
    patterns: Vec<Display>,
    output: Vec<Display>,
    translation: HashMap::<u32, u32>,
}

impl Configuration {
    fn foo(&self) -> Vec<Vec<u32>> {
        self.patterns.iter()
            .map(|pattern| {
                match pattern.lights.keys().count() {
                    2 => vec![1],
                    3 => vec![7],
                    4 => vec![4],
                    5 => vec![2, 3, 5],
                    6 => vec![0, 6, 9],
                    7 => vec![8],
                    _ => vec![],
                }
            })
            .collect()
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
        for x in config.foo() {
            println!("{:?}", x);
        }
    }
}
