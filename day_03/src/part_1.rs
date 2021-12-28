use std::cmp::Ordering;
use std::include_str;

fn thing(b: i32) -> impl Fn(&Vec<i32>) -> (i32, i32) {
    move |ns: &Vec<i32>| -> (i32, i32) {
        let shifted = 2i32.pow(b.try_into().unwrap());
        let n: i32 = ns.iter()
            .map(|n| (n & shifted) >> b)
            .sum();
        match (n * 2).partial_cmp(&(ns.len() as i32)).unwrap() {
            Ordering::Greater => (shifted, 0),
            Ordering::Less => (0, shifted),
            Ordering::Equal => (0, 0),
        }
    }
}

fn main() {
    let ns: Vec<i32> = include_str!("../input.txt")
        .lines()
        .map(|line| i32::from_str_radix(line, 2).unwrap())
        .collect();

    let bits: i32 = 12;

    let bar = (0..bits)
        .map(thing)
        .map(|f| f(&ns))
        .fold((0, 0), |acc, cur| {
            (acc.0 + cur.0, acc.1 + cur.1)
        });

    println!("{}", bar.0 * bar.1);

}
