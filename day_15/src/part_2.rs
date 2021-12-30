//! Dijkstra's algorithm implementation copied from the following Stack Overflow link:
//! https://codereview.stackexchange.com/a/202879
use std::include_str;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

const ROWS: i32 = 100;
const COLS: i32 = 100;
// const ROWS: i32 = 10;
// const COLS: i32 = 10;

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
struct Coord {
    r: u32,
    c: u32,
}

impl Coord {
    fn shift(&self, r: i32, c: i32) -> Option<Coord> {
        let new_r = (self.r as i32) + r;
        let new_c = (self.c as i32) + c;

        if new_r < 0 || new_c < 0 || new_r > (ROWS * 5) - 1 || new_c > (COLS * 5) - 1 {
            None
        } else {
            Some(Coord {
                r: new_r as u32,
                c: new_c as u32,
            })
        }
    }
}

#[derive(Debug)]
struct Visit<V> {
    coord: V,
    distance: u32,
}

impl<V> Ord for Visit<V> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl<V> PartialOrd for Visit<V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<V> PartialEq for Visit<V> {
    fn eq(&self, other: &Self) -> bool {
        self.distance.eq(&other.distance)
    }
}

impl<V> Eq for Visit<V> {}

type Matrix = HashMap<Coord, u32>;

fn get_neighbors(coord: &Coord) -> Vec<Coord> {
    let raw_neighbors = vec![
        coord.shift(-1, 0),
        coord.shift(0, -1),
        coord.shift(0, 1),
        coord.shift(1, 0),
    ];

    raw_neighbors.into_iter().flatten().collect()
}

fn dijkstra(
    start: Coord,
    adjacency_list: &HashMap<Coord, Vec<(Coord, u32)>>,
) -> HashMap<Coord, u32> {
    let mut distances = HashMap::new();
    let mut visited = HashSet::new();
    let mut to_visit = BinaryHeap::new();

    distances.insert(start, 0);
    to_visit.push(Visit {
        coord: start,
        distance: 0,
    });

    while let Some(Visit { coord, distance }) = to_visit.pop() {
        if !visited.insert(coord) {
            continue;
        }

        if let Some(neighbors) = adjacency_list.get(&coord) {
            for (neighbor, cost) in neighbors {
                let new_distance = distance + cost;
                let is_shorter = distances
                    .get(&neighbor)
                    .map_or(true, |&current| new_distance < current);

                if is_shorter {
                    distances.insert(*neighbor, new_distance);
                    to_visit.push(Visit {
                        coord: *neighbor,
                        distance: new_distance,
                    });
                }
            }
        }
    }

    distances
}

fn transform_pair(coord: &Coord, val: u32) -> Vec<(Coord, u32)> {
    let mut transforms = Vec::new();
    for r in 0..5 {
        for c in 0..5 {
            transforms.push((r, c));
        }
    }

    transforms.iter()
        .filter_map(|(r_offset, c_offset)| {
            if let Some(new_coord) = coord.shift(r_offset * ROWS, c_offset * COLS) {
                let additional = (r_offset + c_offset) as u32;
                let mut new_val = (val + (additional)) % 9;
                if new_val == 0 {
                    new_val = 9;
                }
                Some((new_coord, new_val))
            } else {
                None
            }
        })
        .collect()
}

fn main() {
    let lines = include_str!("../input.txt").lines();
    // let lines = include_str!("../example.txt").lines();

    let mut matrix: Matrix = HashMap::new();

    for (r_idx, line) in lines.enumerate() {
        for (c_idx, char) in line.chars().enumerate() {
            let r = r_idx.try_into().unwrap();
            let c = c_idx.try_into().unwrap();
            let val = char.to_digit(10).unwrap();
            matrix.insert(Coord{r, c}, val);
        }
    }

    matrix = matrix.iter()
        .map(|(coord, val)| {
            transform_pair(coord, *val)
        })
        .flatten()
        .collect();

    let adjacency_list = matrix.keys()
        .map(|k| {
            let neighbors = get_neighbors(k);
            let neighbors_mapped = neighbors.iter()
                .map(|n| (*n, *matrix.get(&n).unwrap()))
                .collect::<Vec<(Coord, u32)>>();

            (*k, neighbors_mapped)
        }).collect();

    let start = Coord { r: 0, c: 0 };
    let end = Coord { r: ((ROWS * 5) - 1) as u32, c: ((COLS * 5) - 1) as u32 };

    let foo = dijkstra(start, &adjacency_list);

    println!("{:?}", foo.get(&end).unwrap());
}
