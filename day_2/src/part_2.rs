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
    let mut aim: i32 = 0;

    for n in ns {
        if n.direction == "up" {
            aim -= n.magnitude;
        }
        else if n.direction == "down" {
            aim += n.magnitude;
        }
        else { // n.direction == "forward"
            horizontal += n.magnitude;
            vertical += aim * n.magnitude;
        }
    }

    println!("{}", horizontal * vertical);
}
