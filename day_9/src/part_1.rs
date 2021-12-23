use std::include_str;
use std::collections::HashMap;

const ROWS: usize = 100;
const COLS: usize = 100;

#[derive(Eq, Hash, PartialEq)]
struct Coord {
    r: usize,
    c: usize,
}

impl Coord {
    fn off_board(&self) -> bool {
        self.r < usize::MIN
            || self.r > ROWS - 1
            || self.c < usize::MIN
            || self.c > COLS - 1
    }
}

fn check_neighbors(matrix: &HashMap<Coord, u32>, coord: &Coord) -> u32 {
    let val = matrix.get(coord).unwrap();

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

    if neighbors.iter().all(|neighbor| matrix[coord] < matrix[neighbor]) {
        *val + 1
    } else {
        0
    }
}

fn main() {
    let lines = include_str!("../input.txt")
        .lines();
    let mut matrix: HashMap<Coord, u32> = HashMap::new();

    for (r, line) in lines.enumerate() {
        for (c, char) in line.chars().enumerate() {
            let val = char.to_digit(10u32).unwrap();
            matrix.insert(Coord{r, c}, val);
        }
    }

    let mut low_point_sum: u32 = 0;

    for r in 0..ROWS {
        for c in 0..COLS {
            low_point_sum += check_neighbors(&matrix, &Coord{r, c});
        }
    }

    println!("{}", low_point_sum);
}
