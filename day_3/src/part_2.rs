use std::include_str;
use std::collections::HashMap;

struct Node<T> {
    char: T,
    children: HashMap<i8, Vec<Box<Node<T>>>>,
    sub_items: u32,
}

struct Thing<T> {
    root: Node<T>,
}

impl<String> Thing<String> {
    fn insert(&mut self, item: String) {
        let mut d = item.len(); // digit "length"
        let mut current = self.root;

        for char in item.chars() {
            current.sub_items += d;
            if None(current.children.get(&char)) {
                current.children.insert(char, Node<String> { char: char })
                set_current_dir.c


            }

        }
    }
}


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
