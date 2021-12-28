from functools import reduce

m = {
    0: set("abcefg"),
    1: set("cf"),
    2: set("acdeg"),
    3: set("acdfg"),
    4: set("bcdf"),
    5: set("abdfg"),
    6: set("abdefg"),
    7: set("acf"),
    8: set("abcdefg"),
    9: set("abcdfg"),
}

def p(n):
    return m[n]

def build_set(items):
    return reduce(lambda a, c: a.union(c), (p(8) - s for s in items))

size_5_items = [p(2), p(3), p(5)]
size_6_items = [p(0), p(6), p(9)]

size_5_diff_pins = build_set(size_5_items)
size_6_diff_pins = build_set(size_6_items)

set_a = p(7) - p(1)
set_b_d = p(4) - p(1)
set_d = set_b_d.intersection(size_6_diff_pins)
set_b = set_b_d - set_d

p_0 = p(8) - set_d

set_c_e = size_6_diff_pins.intersection(p_0)
set_c = p(1).intersection(set_c_e)

p_6 = p(8) - set_c

set_e = set_c_e - set_c

p_9 = p(8) - set_e

set_b_e = set_b.union(set_e)
p_3 = p(8) - set_b_e
p_5 = p(8) - set_c_e

set_b_c_e = set_c_e.union(set_b)
set_f = size_5_diff_pins - set_b_c_e
set_b_f = set_b.union(set_f)

p_2 = p(8) - set_b_f

# Assertions
assert p_0 == p(0)
assert p_2 == p(2)
assert p_3 == p(3)
assert p_5 == p(5)
assert p_6 == p(6)
assert p_9 == p(9)
