use std::include_str;

fn fuel_cost(n: i32) -> i32 {
    ((n + 1) * n) / 2
}

fn main() {
    let ns: Vec<i32> = include_str!("../input.txt")
        .lines()
        .flat_map(|line| line.split(","))
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    let max_ns = *ns.iter().max().unwrap();
    let min_ns = *ns.iter().min().unwrap();

    // Let's get weird with closures!
    let cl = |s: i32| -> i32 {
        ns.iter()
            .map(|n: &i32| (n-s).abs())
            .map(fuel_cost)
            .sum()
    };

    let shifts: Vec<i32> = (min_ns..=max_ns)
        .map(cl)
        .collect();

    println!("{}", *shifts.iter().min().unwrap());
}
