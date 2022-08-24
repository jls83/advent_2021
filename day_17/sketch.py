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


def next_position(curr_position: Point, curr_velocity: VelocityVector) -> Tuple[Point, VelocityVector]:
    next_x_position = curr_position.x + curr_velocity.x
    next_y_position = curr_position.y + curr_velocity.y

    next_x_velocity = curr_velocity.x
    if curr_velocity.x > 0:
        next_x_velocity -= 1
    elif curr_velocity.x < 0:
        next_x_velocity += 1

    next_y_velocity = curr_velocity.y - 1

    return Point(next_x_position, next_y_position), VelocityVector(next_x_velocity, next_y_velocity)


def run_trials(target_rectangle: Rectangle, initial_velocity: VelocityVector) -> bool:
    curr_position = Point(0, 0)
    curr_velocity = initial_velocity

    highest_position = curr_position

    while not target_rectangle.past_rectangle(curr_position):
        curr_position, curr_velocity = next_position(curr_position, curr_velocity)

        if curr_position.y > highest_position.y:
            highest_position = curr_position

        if target_rectangle.in_rectangle(curr_position):
            print(f'Highest altitude = {highest_position.y}')
            return True

    return False
