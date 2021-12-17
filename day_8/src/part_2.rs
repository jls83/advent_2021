use std::include_str;
use std::collections::{HashMap, HashSet};

fn build_set(items: &Vec<HashSet<char>>, all_pins_set: &HashSet<char>) -> HashSet<char> {
    items.into_iter()
        .map(|item| all_pins_set - &item)
        .fold(
            HashSet::new(),
            |a, c| {
                a.union(&c).cloned().collect()
            })
}

fn build_map_key(pin_coll: &HashSet<char>) -> String {
    let mut blah: Vec<&char> = pin_coll.into_iter().collect();
    blah.sort();
    blah.iter().map(|c| *c).collect()
}

fn get_pin_mapping(things: Vec<&str>) -> HashMap<String, i32> {
    let mut len_map: HashMap<usize, Vec<HashSet<char>>> = HashMap::new();
    for thing in things {
        let val = len_map.entry(thing.len()).or_insert(Vec::new());
        val.push(thing.chars().collect());
    }

    let size_2_set = &len_map[&2][0];
    let size_3_set = &len_map[&3][0];
    let size_4_set = &len_map[&4][0];
    let size_7_set = &len_map[&7][0];

    let size_5_items = &len_map[&5];
    let size_6_items = &len_map[&6];

    let size_5_diff_pins = build_set(size_5_items, size_7_set);
    let size_6_diff_pins = build_set(size_6_items, size_7_set);

    let set_b_d = size_4_set - size_2_set;
    let set_d = &set_b_d.intersection(&size_6_diff_pins).copied().collect();
    let set_b = &set_b_d - &set_d;

    let p_0 = size_7_set - set_d;

    let set_c_e = size_6_diff_pins.intersection(&p_0).copied().collect();
    let set_c = size_2_set.intersection(&set_c_e).copied().collect();
    let set_e = &set_c_e - &set_c;
    let set_b_e: HashSet<char> = set_b.union(&set_e).cloned().collect();
    let set_b_c_e: HashSet<char> = set_c_e.union(&set_b).cloned().collect();
    let set_f = &size_5_diff_pins - &set_b_c_e;
    let set_b_f = set_b.union(&set_f).cloned().collect();

    let p_2 = size_7_set - &set_b_f;
    let p_3 = size_7_set - &set_b_e;
    let p_5 = size_7_set - &set_c_e;
    let p_6 = size_7_set - &set_c;
    let p_9 = size_7_set - &set_e;

    vec![
        (build_map_key(&p_0), 0),
        (build_map_key(size_2_set), 1),  // p_1
        (build_map_key(&p_2), 2),
        (build_map_key(&p_3), 3),
        (build_map_key(size_4_set), 4),  // p_4
        (build_map_key(&p_5), 5),
        (build_map_key(&p_6), 6),
        (build_map_key(size_3_set), 7), // p_7
        (build_map_key(size_7_set), 8), // p_8
        (build_map_key(&p_9), 9),
    ].into_iter().collect()
}

fn main() {
    let configs: Vec<Vec<&str>> = include_str!("../input.txt")
        .lines()
        .map(|line| {
            line.split(" | ").collect::<Vec<&str>>()[0]
        })
        .map(|s| s.split(" ").collect::<Vec<&str>>())
        .collect();

    let things: Vec<Vec<&str>> = include_str!("../input.txt")
        .lines()
        .map(|line| {
            line.split(" | ").collect::<Vec<&str>>()[1]
        })
        .map(|s| s.split(" ").collect::<Vec<&str>>())
        .collect();

    let mut answer: i32 = 0;

    for (examples, outputs) in configs.into_iter().zip(things.iter()) {
        let mapping = get_pin_mapping(examples);
        let blah: i32 = outputs.iter()
            .map(|output| {
                build_map_key(&output.chars().collect::<HashSet<char>>())
            })
            .map(|output_key| {
                mapping[&output_key]
            })
            .rev()
            .enumerate()
            .map(|(i, n)| {
                n * (10i32.pow(i.try_into().unwrap()))
            })
            .sum();

        answer += blah;
    }

    println!("{}", answer)
}
