def triangular_n_between(a, b):
    x = 0

    while not (a <= ((x * (x - 1)) / 2) <= b):
        x += 1

    return x
