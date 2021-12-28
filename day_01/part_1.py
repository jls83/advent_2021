FILE_NAME = "input.txt"

def f():
    res = 0
    with open(FILE_NAME, 'r') as f:
        prev = float('inf')
        for line in f.readlines():
            curr = int(line)
            if curr > prev:
                res += 1
            prev = curr
    return res

if __name__ == "__main__":
    print(f())
