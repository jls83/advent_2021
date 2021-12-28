use std::include_str;

fn main() {
    let mut ns: Vec<i32> = include_str!("../input.txt")
        .lines()
        .flat_map(|line| line.split(","))
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    for _ in 0..80 {
        let mut new_ns: Vec<i32> = Vec::new();

        for n in &ns {
            if *n == 0 {
                new_ns.push(6);
                new_ns.push(8);
            } else {
                new_ns.push(n-1)
            }
        }

        ns = new_ns
    };

    println!("{}", ns.len());
}
