use std::include_str;
use std::collections::{HashMap, HashSet, VecDeque};

// const ROWS: usize = 5;
// const COLS: usize = 10;
const ROWS: usize = 100;
const COLS: usize = 100;

#[derive(Eq, Hash, PartialEq)]
struct Coord {
    r: usize,
    c: usize,
}

type Matrix = HashMap<Coord, u32>;

fn get_all_neighbors(coord: &Coord) -> Vec<Coord> {
    let mut neighbors: Vec<Coord> = Vec::new();
    if coord.r > usize::MIN {
        neighbors.push(Coord { r: coord.r - 1, c: coord.c });
    }
    if coord.r < ROWS-1 {
        neighbors.push(Coord { r: coord.r + 1, c: coord.c });
    }

    if coord.c > usize::MIN {
        neighbors.push(Coord { r: coord.r, c: coord.c - 1 });
    }
    if coord.c < COLS-1 {
        neighbors.push(Coord { r: coord.r, c: coord.c + 1 });
    }

    neighbors
}

fn is_low_point(matrix: &Matrix, coord: &Coord) -> bool {
    get_all_neighbors(coord).iter()
        .all(|neighbor| matrix[coord] < matrix[neighbor])
}

fn bfs(matrix: &Matrix, start: &Coord) -> u32 {
    let mut deq: VecDeque<Coord> = VecDeque::from(get_all_neighbors(start));
    let mut size: u32 = 0;
    let mut seen: HashSet<Coord> = HashSet::new();

    while !deq.is_empty() {
        let next_coord = deq.pop_front().unwrap();
        if !seen.contains(&next_coord) && matrix[&next_coord] != 9 {
            size += 1;
            for neighbor in get_all_neighbors(&next_coord) {
                deq.push_back(neighbor);
            }
            seen.insert(next_coord);
        }
    }

    size
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

    let mut low_points: Vec<Coord> = Vec::new();

    for r in 0..ROWS {
        for c in 0..COLS {
            let coord = Coord{r, c};
            if is_low_point(&matrix, &coord) {
                low_points.push(coord);
            }
        }
    }

    let mut basin_sizes: Vec<u32> = low_points.iter()
        .map(|low_point| bfs(&matrix, &low_point))
        .collect();

    basin_sizes.sort();

    let result: u32 = basin_sizes.iter()
        .rev()
        .take(3)
        .fold(1, |a, c| a * c);

    println!("{:?}", result);
}
