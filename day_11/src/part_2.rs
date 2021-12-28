use std::include_str;
use std::collections::{HashMap, HashSet, VecDeque};

const ROWS: i32 = 10;
const COLS: i32 = 10;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
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

fn get_neighbors(coord: Coord) -> Vec<Coord> {
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

fn thing(matrix: &Matrix) -> (HashMap<Coord, u32>, usize) {
    let mut new_matrix: Matrix = matrix.clone();

    let mut deq: VecDeque<Coord> = VecDeque::from_iter(matrix.keys().map(|p| *p));
    let mut flashed: HashSet<Coord> = HashSet::new();

    while !deq.is_empty() {
        let coord = deq.pop_front().unwrap();
        if flashed.contains(&coord) {
            continue;
        }
        let val = new_matrix.get_mut(&coord).unwrap();
        *val += 1;

        if *val > 9 {
            flashed.insert(coord);
            for neighbor in get_neighbors(coord) {
                deq.push_back(neighbor);
            }
            *val = 0;
        }
    }

    (new_matrix, flashed.len())
}

fn main() {
    // let lines = include_str!("../example.txt")
    let lines = include_str!("../input.txt")
        .lines();

    let mut matrix: Matrix = HashMap::new();

    for (r, line) in lines.enumerate() {
        for (c, char) in line.chars().enumerate() {
            let val = char.to_digit(10u32).unwrap();
            matrix.insert(Coord{r, c}, val);
        }
    }

    let mut count = 1;

    loop {
        let (new_matrix, next_flashed_count) = thing(&matrix);
        if next_flashed_count == (ROWS * COLS).try_into().unwrap() {
            println!("{}", count);
            break;
        }
        count += 1;
        matrix = new_matrix;
    }
}
