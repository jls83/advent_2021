FILE_NAME = "input.txt"

def generate_ns(file_name):
    with open(FILE_NAME, 'r') as f:
        ns = [int(line) for line in f.readlines()]
    return ns

def f():
    ns = generate_ns(FILE_NAME)
    res = 0
    
    prev = float('inf')
    for i in range(len(ns) - 2):
        curr = sum(ns[i:i+3])
        if curr > prev:
            res += 1
        prev = curr
    return res

def f2():
    ns = generate_ns(FILE_NAME)
    res = 0

    ptr = 0
    prev = sum(ns[:3])

    while ptr < len(ns) - 3:
        curr = prev - ns[ptr] + ns[ptr+3]
        if curr > prev:
            res += 1
        prev = curr
        ptr += 1
    return res

def f3():
    ns = generate_ns(FILE_NAME)
    res = 0

    ptr = 0
    while ptr < len(ns) - 3:
        res += int(ns[ptr] < ns[ptr+3])
        ptr += 1

    return res

def f4():
    ns = generate_ns(FILE_NAME)

    return sum((ns[ptr] < ns[ptr+3]) for ptr in range(len(ns) - 3))

if __name__ == "__main__":
    print(f4())
