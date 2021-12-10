use std::include_str;


fn main() {
    let ns: Vec<i32> = include_str!("../input.txt")
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    let mut counter = 0;
    let mut prev = std::i32::MAX;

    for n in ns {
        if n > prev {
            counter += 1;
        }
        prev = n;
    }
    println!("{}", counter);
}
