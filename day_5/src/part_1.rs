use std::include_str;
use std::collections::HashMap;

#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Point {
    x: i32,
    y: i32,
}

struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn slope_pair(&self) -> (i32, i32) {
        let delta_y = self.end.y - self.start.y;
        let delta_x = self.end.x - self.start.x;

        (delta_y, delta_x)
    }

    fn value_range(&self) -> Vec<Point> {
        match self.slope_pair() {
            // Vertical line
            (_, 0) => {
                (self.start.y..self.end.y+1)
                    .map(|ly| Point { x: self.start.x, y: ly })
                    .collect()
            },
            // Horizontal line
            (0, _) => {
                (self.start.x..self.end.x+1)
                    .map(|lx| Point { x: lx, y: self.start.y })
                    .collect()
            },
            // All other lines
            _ => Vec::new(),
        }
    }
}

fn marshal_pairs<T>(sp: &mut Vec<T>) -> Result<(&T, &T), &'static str> {
    match &sp[..] {
        [a, b] => Ok((a, b)),
        _ => Err("Whoopsie"),
    }
}

fn line_from_str(s: &str) -> Line {
    let mut s_split: Vec<&str> = s.split(" -> ").collect();

    let (a, b) = marshal_pairs::<&str>(&mut s_split).unwrap();

    let mut a_split: Vec<i32> = a.split(",").map(str::parse).map(Result::unwrap).collect();
    let mut b_split: Vec<i32> = b.split(",").map(str::parse).map(Result::unwrap).collect();

    let (ax, ay) = marshal_pairs::<i32>(&mut a_split).unwrap();
    let (bx, by) = marshal_pairs::<i32>(&mut b_split).unwrap();

    let mut points = vec![
        Point { x: *ax, y: *ay },
        Point { x: *bx, y: *by }
    ];

    points.sort();

    let end = points.pop().unwrap();
    let start = points.pop().unwrap();

    Line { start, end }
}

fn main() {
    let lines: Vec<Line> = include_str!("../input.txt")
        .lines()
        .map(line_from_str)
        .collect();


    let touched_points: Vec<Point> = lines.iter()
        .flat_map(|line| line.value_range())
        .collect();

    let mut touch_count: HashMap<Point, i32> = HashMap::new();

    for p in touched_points {
        let val = touch_count.entry(p).or_insert(0);
        *val += 1;
    }

    let count = touch_count.values()
        .filter(|v| **v >= 2)
        .count();

    println!("{}", count);

}
