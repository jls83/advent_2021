use std::include_str;

fn get_most_least_common(ns: &Vec<i32>, bit: u32) -> (Vec<i32>, Vec<i32>) {
    let (zeros, ones): (Vec<i32>, Vec<i32>) = ns.into_iter()
        .partition(|n| *n & 2i32.pow(bit) == 0);

    // More zeros than ones
    if zeros.len() > ones.len() {
        (zeros, ones)
    }
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

    let (mut most_common, mut least_common) = get_most_least_common(&ns, bits-1);

    for bit in (0..bits-1).rev() {
        if most_common.len() == 1 { break; }
        most_common = get_most_least_common(&most_common, bit).0;
    }
    for bit in (0..bits-1).rev() {
        if least_common.len() == 1 { break; }
        least_common = get_most_least_common(&least_common, bit).1;
    }

    println!("{}", most_common[0] * least_common[0]);
}
