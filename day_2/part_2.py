FILE_NAME = "input.txt"

def generate_dirs():
    dirs = []
    with open(FILE_NAME, 'r') as f:
        for line in f.readlines():
            dir, val = line.split(' ')
            val = int(val)
            dirs.append((dir, val))
    return dirs


def f():
    vertical = 0
    horizontal = 0
    aim = 0
    dirs = generate_dirs()

    for dir, val in dirs:
        if dir == 'forward':
            horizontal += val
            vertical += aim * val
        elif dir == 'up':
            aim -= val
        elif dir == 'down':
            aim += val

    return horizontal * vertical

if __name__ == "__main__":
    print(f())

        
