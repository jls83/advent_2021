use std::include_str;

fn main() {
    let ns: Vec<i32> = include_str!("../input.txt")
        .lines()
        .flat_map(|line| line.split(","))
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    let max_ns = *ns.iter().max().unwrap();
    let min_ns = *ns.iter().min().unwrap();

    let shifts: Vec<i32> = (min_ns..=max_ns)
        .map(|s| {
            ns.iter()
                .map(|n| (n-s).abs())
                .sum()
        })
        .collect();

    println!("{}", *shifts.iter().min().unwrap());
}
