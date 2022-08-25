import math
from typing import List, Tuple

class Point:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y

    def __repr__(self):
        return f'<Point: ({self.x}, {self.y})>'

class VelocityVector:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y

    def __repr__(self):
        return f'<VelocityVector: ({self.x}, {self.y})>'

class Rectangle:
    # TODO: Sort? Meh.
    def __init__(self, top_left: Point, bottom_right: Point):
        self.top_left = top_left
        self.bottom_right = bottom_right

    def in_rectangle(self, point: Point) -> bool:
        x_check = self.top_left.x <= point.x <= self.bottom_right.x
        y_check = self.top_left.y <= point.y <= self.bottom_right.y
        return x_check and y_check

    def past_rectangle(self, point: Point) -> bool:
        """
        If the given point is "past" the rectangle.
        """
        x_check = point.x > self.bottom_right.x
        y_check = point.y < self.bottom_right.y
        return x_check or y_check


def create_triangular_num(n: int) -> int:
    return (n * (n + 1)) // 2


def get_triangular_base(n: int) -> int:
    return math.floor(math.sqrt(n * 2))


def is_triangular_num(n: int) -> bool:
    base = get_triangular_base(n)
    return n == create_triangular_num(base)


def get_fall_distances(max_height: int, start: int, end: int) -> List[int]:
    start_normalized = abs(start - max_height)
    end_normalized = abs(end - max_height)
    return [n for n in range(start_normalized, end_normalized + 1) if is_triangular_num(n)]


def get_possible_y_velocities(start: int, end: int, max_velocity: int=10) -> List[int]:
    return [i for i in range(max_velocity) if get_fall_distances(create_triangular_num(i), start, end)]


def get_min_x_velocity(min_distance: int) -> int:
    return get_triangular_base(min_distance)


def next_x_position_and_velocity(curr_position: Point, curr_velocity: VelocityVector) -> Tuple[int, int]:
    next_x_position = curr_position.x + curr_velocity.x
    next_x_velocity = curr_velocity.x
    if curr_velocity.x > 0:
        next_x_velocity -= 1
    elif curr_velocity.x < 0:
        next_x_velocity += 1

    return next_x_position, next_x_velocity


def next_y_position_and_velocity(curr_position: Point, curr_velocity: VelocityVector) -> Tuple[int, int]:
    next_y_position = curr_position.y + curr_velocity.y
    next_y_velocity = curr_velocity.y - 1

    return next_y_position, next_y_velocity


def next_position(curr_position: Point, curr_velocity: VelocityVector) -> Tuple[Point, VelocityVector]:
    next_x_position, next_x_velocity = next_x_position_and_velocity(curr_position, curr_velocity)
    next_y_position, next_y_velocity = next_y_position_and_velocity(curr_position, curr_velocity)

    return Point(next_x_position, next_y_position), VelocityVector(next_x_velocity, next_y_velocity)


def run_trial(target_rectangle: Rectangle, initial_velocity: VelocityVector) -> Tuple[bool, int]:
    curr_position = Point(0, 0)
    curr_velocity = initial_velocity

    highest_position = curr_position

    while not target_rectangle.past_rectangle(curr_position):
        curr_position, curr_velocity = next_position(curr_position, curr_velocity)

        if curr_position.y > highest_position.y:
            highest_position = curr_position

        if target_rectangle.in_rectangle(curr_position):
            return True, highest_position.y

    return False, 0


def thinger(point_1: Point, point_2: Point):
    rect = Rectangle(point_1, point_2)
    min_x_val, max_x_val = sorted([point_1.x, point_2.x])
    closest_y_val, furthest_y_val = sorted([point_1.y, point_2.y], key=abs)
    min_x_velocity = get_min_x_velocity(min_x_val)

    max_y_velocity = abs(furthest_y_val)

    possible_y_velocities = get_possible_y_velocities(closest_y_val, furthest_y_val, max_velocity=max_y_velocity)

    max_height = float('-inf')

    for x_velocity in range(min_x_velocity, max_x_val+1):
        # for y_velocity in possible_y_velocities:
        for y_velocity in range(1, max_y_velocity):
            found, height = run_trial(rect, VelocityVector(x_velocity, y_velocity))

            if not found:
                continue

            max_height = max(max_height, height)

    print(max_height)
