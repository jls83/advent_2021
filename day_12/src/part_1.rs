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

fn dfs<'a>(cave_map: CaveMap<'a>, start: Cave<'a>) -> u32 {
    let mut deq: VecDeque<(Cave, Vec<Cave>)> = VecDeque::new();
    deq.push_front((start, Vec::new()));

    let mut count: u32 = 0;

    while !deq.is_empty() {
        let (next_cave, path) = deq.pop_front().unwrap();
        for neighbor in &cave_map[&next_cave] {
            if neighbor.name == "end" {
                count += 1;
                continue;
            }
            let small_seen_here: HashSet<Cave> = path.iter()
                .filter(|cave| cave.size == CaveSize::SMALL)
                .map(|c| *c)
                .collect();
            if small_seen_here.contains(neighbor) {
                continue;
            } else if neighbor.name == "start" {
                continue;
            }
            let mut next_path = path.to_vec();
            next_path.push(*neighbor);
            deq.push_front((*neighbor, next_path));
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
