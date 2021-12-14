use std::include_str;
use std::collections::{HashMap, HashSet};

#[derive(Default)]
struct Board {
    map: HashMap<i32, (usize, usize)>,
    uncalled_numbers: HashSet<i32>,
    called_by_row: HashMap<usize, HashSet<usize>>,
    called_by_col: HashMap<usize, HashSet<usize>>,
}

impl Board {
    fn insert(&mut self, r: usize, c: usize, val: i32) {
        self.map.insert(val, (r, c));
        self.uncalled_numbers.insert(val);
    }

    fn mark_number(&mut self, val: &i32) {
        match self.map.get(val) {
            Some((r, c)) => {
                let called_by_row = self.called_by_row.entry(*r)
                    .or_insert(HashSet::new());
                let called_by_col = self.called_by_col.entry(*c)
                    .or_insert(HashSet::new());

                called_by_row.insert(*c);
                called_by_col.insert(*r);

                self.uncalled_numbers.remove(val);
            },
            None => (),
        }
    }

    fn is_filled(&self) -> bool {
        // I want this to just be a constant, but we're not allowed :(
        let all_ns: HashSet<usize> = (0..5).collect();

        self.called_by_col.values().any(|s| *s == all_ns)
            || self.called_by_row.values().any(|s| *s == all_ns)
    }

    fn unmarked_sum(&self) -> i32 {
        self.uncalled_numbers.iter().sum()
    }
}

fn parse_input(line: &str) -> Vec<i32> {
    let foo: Vec<i32> = line
        .split(",")
        .map(str::parse)
        .map(Result::unwrap)
        .collect();
    foo
}

fn parse_board(chunk: &[&str]) -> Board {
    let mut board: Board = Default::default();
    for (r, row) in chunk.iter().enumerate() {
        let vals: Vec<i32> = row.split(" ")
            .filter(|&x| !x.is_empty())
            .map(str::parse)
            .map(Result::unwrap)
            .collect();

        for (c, val) in vals.iter().enumerate() {
            board.insert(r, c, *val);
        }
    }

    board
}

fn main() {
    let lines: Vec<&str> = include_str!("../input.txt")
        .lines()
        .filter(|line| !line.is_empty())
        .collect();

    let input = parse_input(lines[0]);

    let mut boards: Vec<Board> = lines[1..].chunks(5)
        .map(parse_board)
        .collect();

    for n in input {
        for board in &mut boards {
            board.mark_number(&n);
            if board.is_filled() {
                println!("Found filled board {}", n * board.unmarked_sum());
                return;
            }
        }
    }
}
