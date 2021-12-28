use std::include_str;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum CaveSize {
    SMALL,
    LARGE,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Cave<'a> {
    name: &'a str,
    size: CaveSize,
}

type CaveMap<'a> = HashMap<Cave<'a>, HashSet<Cave<'a>>>;

fn create_cave(name: &str) -> Cave {
    if name.to_uppercase() == name {
        Cave { name, size: CaveSize::LARGE }
    } else {
        Cave { name, size: CaveSize::SMALL }
    }
}

fn create_caves(line: &str) -> (Cave, Cave) {
    let caves: Vec<&str> = line.split("-").collect();
    let (cave_a_str, cave_b_str) = (caves[0], caves[1]);

    (create_cave(cave_a_str), create_cave(cave_b_str))
}

fn seen_multiple_small_old<'a>(path: &Vec<Cave<'a>>) -> bool {
    let mut seen_count: HashMap<Cave, u32> = HashMap::new();

    for cave in &*path {
        if cave.size == CaveSize::LARGE {
            continue;
        }
        let val = seen_count.entry(*cave).or_insert(0);
        *val += 1
    }

    // All less than one?
    if seen_count.iter().all(|(_, v)| *v <= 1) {
        return false;
    } // Only one equal to 2?
    else if seen_count.iter().filter(|(_, v)| **v == 2).count() == 1 {
        return false;
    } else {
        return true;
    }
}

fn seen_multiple_small<'a>(path: &Vec<Cave<'a>>) -> bool {
    let mut seen_count: HashMap<Cave, u32> = HashMap::new();
    let mut blah: HashSet<Cave> = HashSet::new();

    for cave in &*path {
        if cave.size == CaveSize::LARGE {
            continue;
        }
        let val = seen_count.entry(*cave).or_insert(0);
        *val += 1;

        if *val > 2 {
            return true;
        } else if *val > 1 {
            blah.insert(*cave);
        }

        if blah.len() > 1 {
            return true;
        }
    }
    
    return false;
}

fn print_path<'a>(path: &Vec<Cave<'a>>) {
    let path_str = path.iter().map(|cave| cave.name).collect::<Vec<&str>>().join(",");
    println!("{},end", path_str);
}

fn check_thinger<'a>(thinger: &HashMap<Cave<'a>, u32>) -> bool {
    let mut foo: HashSet<Cave> = HashSet::new();
    for (k, v) in thinger.iter() {
        if k.size == CaveSize::LARGE {
            continue;
        } else if *v > 2 {
            return true;
        } else if *v == 2 {
            foo.insert(*k);
        }
    }
    foo.len() > 1
}

fn dfs<'a>(cave_map: CaveMap<'a>, start: Cave<'a>) -> u32 {
    // let mut deq: VecDeque<(Cave, Vec<Cave>)> = VecDeque::new();
    // deq.push_front((start, vec![start]));

    let mut deq: VecDeque<(Cave, HashMap<Cave, u32>)> = VecDeque::new();
    deq.push_front((start, HashMap::new()));

    let mut count: u32 = 0;

    while !deq.is_empty() {
        let (next_cave, thinger) = deq.pop_front().unwrap();
        for neighbor in &cave_map[&next_cave] {
            if neighbor.name == "end" {
                count += 1;
                continue;
            }

            let mut next_thinger = thinger.clone();
            let val = next_thinger.entry(*neighbor).or_insert(0);
            *val += 1;

            if check_thinger(&next_thinger) {
                continue;
            } else if neighbor.name == "start" {
                continue;
            }
            deq.push_front((*neighbor, next_thinger));
        }
    }

    count
}

fn main() {
    // let lines = include_str!("../example_3.txt")
    let lines = include_str!("../input.txt")
        .lines();

    let mut cave_map: CaveMap = HashMap::new();

    let cave_edges = lines.map(create_caves);

    for (cave_a, cave_b) in cave_edges {
        let cave_a_neighbors = cave_map.entry(cave_a).or_insert(HashSet::new());
        if cave_a.name != "end" {
            cave_a_neighbors.insert(cave_b);
        }

        let cave_b_neighbors = cave_map.entry(cave_b).or_insert(HashSet::new());
        if cave_a.name != "start" {
            cave_b_neighbors.insert(cave_a);
        }
    }

    let start_cave = cave_map.keys()
        .filter(|cave| cave.name == "start")
        .map(|c| *c)
        .collect::<Vec<Cave>>()[0];

    let res = dfs(cave_map, start_cave);

    println!("{}", res);
}
