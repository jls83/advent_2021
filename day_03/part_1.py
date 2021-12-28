BITS = 12


def f(ns):
    delta = 0
    epsilon = 0

    for b in range(BITS):
        foo = sum(((n & (2 ** b)) >> b) for n in ns)

        if foo > len(ns) // 2:
            delta |= 2 ** b
        else:
            epsilon |= 2 ** b

    return delta * epsilon

if __name__ == "__main__":
    with open('input.txt', 'r') as fi:
        foo = [int(i, 2) for i in fi.readlines()]
    print(f(foo))
