from enum import Enum

def digit_len(n):
    c = 0
    while n > 1:
        n >>= 1
        c += 1
    return c + 1

def get_bits(n, start, length):
    mask = ((1 << length) - 1) << start
    return n & mask

def get_mask(n, size):
    l = digit_len(n)
    mask_base = (1 << size) - 1
    return mask_base << (l - size)

110100101111111000101000
110000000000000000000000


def get_version_number_and_type_id(n):
    l = digit_len(n)
    version_number_type_id = (n & get_mask(n, 6)) >> (l - 6)

    version_number = (version_number_type_id & get_mask(version_number_type_id, 3)) >> 3
    type_id = version_number_type_id & 7

    return version_number, type_id

def clear_first_d(n, size):
    l = digit_len(n)
    subtract = (1 << size) - 1
    return n - (subtract << (l - size))

def parse_literal(n):
    pass

def parse_operator_packet(n):
    l = digit_len(n)
    length_type_id = (n & get_mask(n, 1)) >> (l - 1)

    next_n = clear_first_d(n, 1)
    next_l = digit_len(l)

    if length_type_id == 0:
        sub_packet_length = (next_n & get_mask(next_n, 15)) >> (next_l - 15)

    pass

def foo(n):
    version_number, type_id = get_version_number_and_type_id(n)

    next_n = clear_first_d(n, 6)

    if type_id == 4:
        val = parse_literal(next_n) 
        print(val)
    else:
        blah = parse_operator_packet(next_n)

