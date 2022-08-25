use std::cmp;

#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}


#[derive(Clone)]
struct VelocityVector {
    x: i32,
    y: i32,
}

struct ClosestFurthest {
    closest: i32,
    furthest: i32,
}

fn closest_furthest(foo: i32, bar: i32) -> ClosestFurthest {
    let mut vals = vec![foo, bar];
    vals.sort_by(|a, b| { a.abs().cmp(&b.abs()) });

    ClosestFurthest { closest: vals[0], furthest: vals[1] }
}

impl Rectangle {
    fn in_rectangle(&self, point: Point) -> bool {
        let x_check = self.top_left.x <= point.x && point.x <= self.bottom_right.x;
        let y_check = self.top_left.y <= point.y && point.y <= self.bottom_right.y;

        x_check && y_check
    }

    fn past_rectangle(&self, point: &Point) -> bool {
        let x_check = point.x > self.bottom_right.x;
        let y_check = point.y < self.bottom_right.y;

        x_check || y_check
    }

    fn closest_furthest_x(&self) -> ClosestFurthest {
        closest_furthest(self.top_left.x, self.bottom_right.x)
    }

    fn closest_furthest_y(&self) -> ClosestFurthest {
        closest_furthest(self.top_left.y, self.bottom_right.y)
    }
}

fn next_position(curr_position: &Point, curr_velocity: VelocityVector) -> (Point, VelocityVector) {
    let next_position = Point {
        x: curr_position.x + curr_velocity.x,
        y: curr_position.y + curr_velocity.y,
    };

    let mut next_x_velocity = curr_velocity.x;
    if next_x_velocity > 0 {
        next_x_velocity -= 1;
    } else if next_x_velocity < 0 {
        next_x_velocity += 1;
    }

    let next_y_velocity = curr_velocity.y - 1;

    (next_position, VelocityVector { x: next_x_velocity, y: next_y_velocity })
}

fn get_triangular_base(n: i32) -> i32 {
    let float_n = (n as f64) * 2.0;
    return float_n.sqrt().floor() as i32;
}

fn get_min_x_velocity(min_distance: i32) -> i32 {
    get_triangular_base(min_distance)
}

fn run_trial(target_rectangle: &Rectangle, initial_velocity: VelocityVector) -> Option<i32> {
    let mut curr_position = Point { x: 0, y: 0 };
    let mut curr_velocity = initial_velocity.clone();

    let mut max_height = i32::MIN;

    while !target_rectangle.past_rectangle(&curr_position) {
        (curr_position, curr_velocity) = next_position(&curr_position, curr_velocity);

        max_height = cmp::max(max_height, curr_position.y);

        if target_rectangle.in_rectangle(curr_position) {
            return Some(max_height);
        }
    }

    None
}

fn run_trials(rectangle: Rectangle) {
    let xs = rectangle.closest_furthest_x();
    let ys = rectangle.closest_furthest_y();

    let min_x_velocity = get_min_x_velocity(xs.closest);
    let max_y_velocity = ys.furthest.abs();

    let mut max_height = i32::MIN;

    for x in min_x_velocity..=xs.furthest {
        for y in 1..=max_y_velocity {
            if let Some(height) = run_trial(&rectangle, VelocityVector { x, y }) {
                max_height = cmp::max(max_height, height);
            }
        }
    }

    println!("{}", max_height);
}

fn main() {
    // let top_left = Point { x: 20, y: -10 };
    // let bottom_right = Point { x: 30, y: -5 };
    let top_left = Point { x: 230, y: -107 };
    let bottom_right = Point { x: 283, y: -57 };

    run_trials(Rectangle { top_left, bottom_right });
}
