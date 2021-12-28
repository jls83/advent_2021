use std::include_str;

fn main() {
    let ns: Vec<i32> = include_str!("../input.txt")
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    let result: i32 = (0..ns.len()-1)
        .map(|idx| (ns[idx+1] > ns[idx]) as i32)
        .sum();

    println!("{}", result);
}
