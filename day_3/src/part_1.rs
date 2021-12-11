use std::include_str;


fn main() {
    let bits = 12;

    let ns: Vec<i32> = include_str!("../input.txt")
        .lines()
        .map(|line| i32::from_str_radix(line, 2).unwrap())
        .collect();

    let mut delta: i32 = 0;
    let mut epsilon: i32 = 0;

    for b in 0..bits {
        let thing = 2i32.pow(b);
        let foo: i32 = ns.iter()
            .map(|n| (n & thing) >> b)
            .sum();

        if foo > (ns.len() / 2).try_into().unwrap() {
            delta |= thing;
        } else {
            epsilon |= thing;
        }
    }

    println!("{}", delta * epsilon);

}
