use std::include_str;

struct Thing {
    direction: String,
    magnitude: i32,
}

fn main() {
    let ns = include_str!("../input.txt")
        .lines()
        .map(|line| {
            let mut split = line.split(" ");
            let direction = split.next().unwrap().to_string();
            let magnitude = split.next().unwrap().parse::<i32>().unwrap();

            Thing { direction, magnitude }
        });

    let mut horizontal: i32 = 0;
    let mut vertical: i32 = 0;

    for n in ns {
        let (horiz_delta, verti_delta) = match n.direction.as_str() {
            "forward" => (n.magnitude, 0),
            "up" => (0, n.magnitude * -1),
            "down" => (0, n.magnitude),
            _ => (0, 0),
        };
        horizontal += horiz_delta;
        vertical += verti_delta;
    }

    println!("{}", horizontal * vertical);
}
