use std::include_str;
use std::collections::{HashMap, HashSet};

// const ROWS: i32 = 100;
// const COLS: i32 = 100;
const ROWS: i32 = 10;
const COLS: i32 = 10;

#[derive(Copy, Clone, Debug, Ord, Eq, Hash, PartialEq, PartialOrd)]
struct Coord {
    r: usize,
    c: usize,
}

impl Coord {
    fn shift(&self, r: i32, c: i32) -> Option<Coord> {
        let new_r = (self.r as i32) + r;
        let new_c = (self.c as i32) + c;

        if new_r < 0 || new_c < 0 || new_r > ROWS - 1 || new_c > COLS - 1 {
            None
        } else {
            Some(Coord {
                r: new_r as usize,
                c: new_c as usize,
            })
        }
    }
}

type Matrix = HashMap<Coord, u32>;

fn get_neighbors(coord: &Coord) -> HashSet<Coord> {
    let raw_neighbors = vec![
        coord.shift(-1, -1),
        coord.shift(-1, 0),
        coord.shift(-1, 1),
        coord.shift(0, -1),
        coord.shift(0, 1),
        coord.shift(1, -1),
        coord.shift(1, 0),
        coord.shift(1, 1),
    ];

    raw_neighbors.into_iter().flatten().collect()
}

fn get_min_key(dist: &HashMap<Coord, u32>, q: &HashSet<Coord>) -> Coord {
    let min_key = q.iter()
        .map(|c| (c, dist.get(c).unwrap()))
        .min_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _v)| k)
        .unwrap();

    *min_key
}

fn dijkstra(matrix: Matrix, source: Coord, target: Coord) -> Vec<Coord> {
    let mut dist: HashMap<Coord, u32> = HashMap::new();
    let mut prev: HashMap<Coord, Option<Coord>> = HashMap::new();

    let mut q: HashSet<Coord> = HashSet::new();

    for v in matrix.keys() {
        dist.insert(*v, u32::MAX);
        prev.insert(*v, None);

        q.insert(*v);
    }

    dist.insert(source, 0);

    while !q.is_empty() {
        let mut min_key = get_min_key(&dist, &q);

        q.remove(&min_key);

        if min_key == target {
            let mut res: Vec<Coord> = Vec::new();
            while let Some(p) = prev[&min_key] {
                res.push(p);
                min_key = p;
            }
            return res;
        }

        let neighbors: HashSet<Coord> = get_neighbors(&min_key).intersection(&q).map(|x| *x).collect();

        println!("{:?}", neighbors);

        for neighbor in neighbors {
            let dist_val = dist.entry(min_key).or_default();
            let alt = *dist_val + matrix[&neighbor];

            // println!("{:?} {:?} {:?}", min_key, neighbor, alt);

            if alt >= *dist_val {
                continue;
            }

            *dist_val = alt;

            let prev_val = prev.entry(min_key).or_default();
            *prev_val = Some(min_key);
        }
    }

    vec![]
}

fn main() {
    // let lines = include_str!("../input.txt").lines();
    let lines = include_str!("../example.txt").lines();

    let mut matrix: Matrix = HashMap::new();

    for (r, line) in lines.enumerate() {
        for (c, char) in line.chars().enumerate() {
            let val = char.to_digit(10u32).unwrap();
            matrix.insert(Coord{r, c}, val);
        }
    }

    let foo = dijkstra(
        matrix, 
        Coord {r: 0, c: 0 },
        Coord {r: (ROWS-1) as usize, c: (COLS-1) as usize});

    println!("{:?}", foo);
}
