import math
from typing import Tuple


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


######

def get_min_velocity(measure: int) -> int:
    return math.floor(math.sqrt(measure * 2))


def get_max_time(measure: int, velocity: int) -> int:
    t = 0
    while measure > 0:
        measure -= velocity
        velocity -= 1
        t += 1

    return t

def pos_y(initial_velocity: int, t: int, initial_position: int = 0) -> int:
    gravity_sum = (t * (t - 1)) // 2  # TODO: comment
    return initial_position + (initial_velocity * t) - gravity_sum

def pos_x(initial_velocity: int, t: int, initial_position: int = 0) -> int:
    is_negative = (initial_velocity < 0)
    velocity = abs(initial_velocity)

    distance_travelled = 0
    while t > 0 and velocity > 0:
        distance_travelled += velocity
        velocity -= 1
        t -= 1

    if is_negative:
        distance_travelled *= -1

    return distance_travelled + initial_position


def foo(initial_velocity: int, position: int):
    b = ((2 * initial_velocity) + 1) / 2
    discrim = math.sqrt((b ** 2) + (2 ** position))
    return b+discrim, b-discrim

def x_velo_in_range(velocity: int, start: int, end: int):
    t = 0
    position = 0
    res = []
    while position < end:
        if start <= position <= end:
            res.append(t)
        next_position = position + velocity
        if next_position == position:
            return res, True
        position = next_position
        velocity -= 1
        t += 1

    return res, False

def get_valid_x_velos(min_velocity: int, start: int, end: int):
    blah = ((i, x_velo_in_range(i, start, end)) for i in range(min_velocity, end))
    d = {i: velos for i, velos in blah if velos[0]}
    return d


def run_trial(target_rectangle: Rectangle, initial_velocity: VelocityVector) -> Tuple[bool, int]:
    curr_position = Point(0, 0)
    curr_velocity = initial_velocity

    highest_position = curr_position

    while not target_rectangle.past_rectangle(curr_position):
        curr_position, curr_velocity = next_position(curr_position, curr_velocity)

        if curr_position.y > highest_position.y:
            highest_position = curr_position

        if target_rectangle.in_rectangle(curr_position):
            print(f'Highest altitude = {highest_position.y}')
            return True, highest_position.y

    return False, 0

