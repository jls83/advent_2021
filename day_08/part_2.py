from collections import defaultdict
from functools import reduce

def build_set(items, all_pins_set):
    return reduce(lambda a, c: a.union(c), (all_pins_set - s for s in items))

def f(things):
    len_map = defaultdict(list)
    for thing in things:
        len_map[len(thing)].append(set(thing))

    # TODO: Assert length?
    size_2_set = len_map[2][0]
    size_3_set = len_map[3][0]
    size_4_set = len_map[4][0]
    size_7_set = len_map[7][0]

    size_5_items = len_map[5]
    size_6_items = len_map[6]

    size_5_diff_pins = build_set(size_5_items, size_7_set)
    size_6_diff_pins = build_set(size_6_items, size_7_set)

    set_b_d = size_4_set - size_2_set
    set_d = set_b_d.intersection(size_6_diff_pins)
    set_b = set_b_d - set_d

    p_0 = size_7_set - set_d

    set_c_e = size_6_diff_pins.intersection(p_0)
    set_c = size_2_set.intersection(set_c_e)
    set_e = set_c_e - set_c
    set_b_e = set_b.union(set_e)
    set_b_c_e = set_c_e.union(set_b)
    set_f = size_5_diff_pins - set_b_c_e
    set_b_f = set_b.union(set_f)

    p_2 = size_7_set - set_b_f
    p_3 = size_7_set - set_b_e
    p_5 = size_7_set - set_c_e
    p_6 = size_7_set - set_c
    p_9 = size_7_set - set_e

    return {
        ''.join(sorted(p_0)): 0,
        ''.join(sorted(size_2_set)): 1,  #p_1
        ''.join(sorted(p_2)): 2,
        ''.join(sorted(p_3)): 3,
        ''.join(sorted(size_4_set)): 4,  # p_4
        ''.join(sorted(p_5)): 5,
        ''.join(sorted(p_6)): 6,
        ''.join(sorted(size_3_set)): 7,  # p_7
        ''.join(sorted(size_7_set)): 8,  # p_8
        ''.join(sorted(p_9)): 9,
    }

if __name__ == "__main__":
    with open("input.txt", "r") as fi:
        foos = (line.strip().split(" | ") for line in fi.readlines())
        aaa = [(foo[0].split(" "), foo[1].split(" ")) for foo in foos]

    c = 0

    for examples, outputs in aaa:
        pin_map = f(examples)
        outputs_sorted = ("".join(sorted(v)) for v in outputs)
        output_values = [pin_map[s] for s in outputs_sorted]
        output_n = sum(n * (10 ** i) for i, n in enumerate(reversed(output_values)))
        c += output_n

    print(c)

