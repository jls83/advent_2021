FILE_NAME = "input.txt"

def f():
    vertical = 0
    horizontal = 0
    dirs = []
    with open(FILE_NAME, 'r') as f:
        for line in f.readlines():
            dir, val = line.split(' ')
            val = int(val)
            dirs.append((dir, val))

    for dir, val in dirs:
        if dir == 'forward':
            horizontal += val
        elif dir == 'up':
            vertical -= val
        elif dir == 'down':
            vertical += val

    return horizontal * vertical

if __name__ == "__main__":
    print(f())

        
