use std::include_str;

fn get_most_least_common(ns: &Vec<i32>, bit: u32) -> (Vec<i32>, Vec<i32>) {
    let mut ones: Vec<i32> = Vec::new();
    let mut zeros: Vec<i32> = Vec::new();

    let thing = 2i32.pow(bit);

    for n in ns {
        if (n & thing) == 0 {
            zeros.push(*n);
        } else {
            ones.push(*n);
        }
    }

    // More zeros than ones
    if zeros.len() > ones.len() {
        (zeros, ones)
    }
    // More ones than zeros
    // Equal number, ones are the "most common" by default
    else { // if ones.len() >= zeros.len() {
        (ones, zeros)
    }

}

fn main() {
    let bits = 12;

    let ns: Vec<i32> = include_str!("../input.txt")
        .lines()
        .map(|line| i32::from_str_radix(line, 2).unwrap())
        .collect();

    let mut current_bit = bits - 1;
    let (mut most_common, mut least_common) = get_most_least_common(&ns, current_bit);

    while most_common.len() != 1 {
        current_bit -= 1;
        most_common = get_most_least_common(&most_common, current_bit).0;
    }

    let mut current_bit = bits - 1;
    while least_common.len() != 1 {
        current_bit -= 1;
        least_common = get_most_least_common(&least_common, current_bit).1;
    }

    println!("{}", most_common[0] * least_common[0]);
}
