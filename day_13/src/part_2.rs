use std::cmp;
use std::include_str;
use std::collections::HashSet;

#[derive(Eq, Hash, PartialEq)]
struct Coord {
    r: usize,
    c: usize,
}

#[derive(Eq, Hash, PartialEq)]
enum Axis {
    HORIZONTAL,
    VERTICAL,
}

struct Fold {
    location: usize,
    axis: Axis,
}

type Matrix = HashSet<Coord>;

fn get_coord_from_line(line: &str) -> Coord {
    let line_split: Vec<usize> = line.split(",")
        .map(str::parse)
        .map(Result::unwrap)
        .collect();
    let (r, c) = (line_split[0], line_split[1]);
    
    Coord {r, c}
}

fn get_fold_from_line(line: &str) -> Fold {
    let definition = line.rsplit(' ').next().unwrap();
    let foo: Vec<&str> = definition.split('=').collect();
    let (axis_str, location_str) = (foo[0], foo[1]);

    let location: usize = location_str.parse().unwrap();

    if axis_str == "x" {
        Fold { axis: Axis::HORIZONTAL, location }
    } else {
        Fold { axis: Axis::VERTICAL, location }
    }
}

fn transform_coord(coord: &Coord, fold: &Fold) -> Option<Coord> {
    let r = coord.r;
    let c = coord.c;

    if fold.axis == Axis::HORIZONTAL && r > fold.location {
        Some(Coord { r: (2 * fold.location) - r, c })
    } else if fold.axis == Axis::HORIZONTAL && r == fold.location {
        None
    } else if fold.axis == Axis::VERTICAL && c > fold.location {
        Some(Coord { r, c: (2 * fold.location) - c })
    } else if fold.axis == Axis::VERTICAL && c == fold.location {
        None
    } else {
        Some(Coord { r, c })
    }
}

fn fold_matrix(matrix: &Matrix, fold: &Fold) -> Matrix {
    let f = |old_coord| transform_coord(old_coord, fold);

    matrix.iter()
        .filter_map(f)
        .collect()
}

fn print_matrix(matrix: &Matrix) {
    // get the max rows & cols from the matrix
    let mut rows = 0;
    let mut cols = 0;
    for coord in matrix {
        rows = cmp::max(rows, coord.r);
        cols = cmp::max(cols, coord.c);
    }

    // for each row/col, print out the appropriate "pixel". yes, i know these are backwards.
    for c in 0..=cols {
        let pixel_row = (0..=rows)
            .map(|r| {
                let coord = Coord { r, c };
                if matrix.contains(&coord) {
                    "#"
                } else {
                    " "
                }
            })
            .collect::<Vec<&str>>()
            .join("");
        println!("{}", pixel_row);
    }
}

fn main() {
    let lines = include_str!("../input.txt").lines();
    // let lines = include_str!("../example.txt").lines();
    let (fold_strs, coord_strs): (Vec<&str>, Vec<&str>) = lines.partition(|&line| line.starts_with("f"));

    let coords = coord_strs.into_iter().filter(|line| *line != "").map(get_coord_from_line);
    let matrix: Matrix = HashSet::from_iter(coords);

    let folds: Vec<Fold> = fold_strs.into_iter()
        .map(get_fold_from_line)
        .collect();

    let final_matrix = folds.iter().fold(matrix, |acc, cur| fold_matrix(&acc, cur));

    print_matrix(&final_matrix);
}
