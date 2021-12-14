use std::include_str;
use std::collections::HashMap;

fn born_now(age: i64, count: i64) -> i64 {
    if age >= 8 && (age-8) % 7 == 0 {
        count
    } else {
        0
    }
}

fn main() {
    let ns: Vec<i64> = include_str!("../input.txt")
        .lines()
        .flat_map(|line| line.split(","))
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    let mut births_by_time: HashMap<i64, i64> = HashMap::new();

    for n in &ns {
        let birth_time = (15 - *n) * -1;
        let val = births_by_time.entry(birth_time).or_insert(0);
        *val += 1;
    }

    for current_time in 0..256 {
        let new_births: i64 = births_by_time.iter()
            .map(|(birth_time, count)| {
                born_now(current_time - birth_time, *count)
            })
            .sum();

        // New births are actually one cycle ahead.
        births_by_time.insert(current_time+1, new_births);
    };

    let res: i64 = births_by_time.values().sum();

    println!("{}", res);
}
