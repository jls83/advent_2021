import math
from functools import reduce

class Packet(object):
    _OPERATOR_TYPE_MAP = {
        0: 'sum',
        1: 'product',
        2: 'minimum',
        3: 'maximum',
        4: 'literal',
        5: 'greater_than',
        6: 'less_than',
        7: 'equal_to',
    }
    def __init__(self, version_number, type_id):
        self.version_number = version_number
        self.type_id = type_id

        self.value = None
        self.subpackets = []

    @property
    def version_sum(self):
        if self.value is not None:
            return self.version_number
        return self.version_number + sum(sp.version_sum for sp in self.subpackets)

    @property
    def operator(self):
        return self._OPERATOR_TYPE_MAP[self.type_id]

    def evaluate(self):
        if self.operator == 'literal':
            return self.value
        elif self.operator == 'sum':
            return sum(sp.evaluate() for sp in self.subpackets)
        elif self.operator == 'product':
            return reduce(lambda a, c: a * c, (sp.evaluate() for sp in self.subpackets))
        elif self.operator == 'minimum':
            return min(sp.evaluate() for sp in self.subpackets)
        elif self.operator == 'maximum':
            return max(sp.evaluate() for sp in self.subpackets)
        elif self.operator == 'greater_than':
            subpacket_values = [sp.evaluate() for sp in self.subpackets]
            return int(subpacket_values[0] > subpacket_values[1])
        elif self.operator == 'less_than':
            subpacket_values = [sp.evaluate() for sp in self.subpackets]
            return int(subpacket_values[0] < subpacket_values[1])
        elif self.operator == 'equal_to':
            subpacket_values = [sp.evaluate() for sp in self.subpackets]
            return int(subpacket_values[0] == subpacket_values[1])

def hex_str_to_bin_str(hex_str):
    int_val = int('0x' + hex_str, 16)
    return format(int_val, '0{}b'.format(len(hex_str) * 4))

def digit_count_with_padding(n):
    log_2_x = math.log2(n)
    x = log_2_x % 4
    return int((math.log2(n) - x) + 4)

def get_substring_as_int(bin_str, start, length):
    return int('0b' + bin_str[start:start+length], 2)

def parse_literal(bin_str, start):
    res = 0
    i = start
    next_i = i
    while True:
        res <<=4
        res += get_substring_as_int(bin_str, i+1, 4)
        next_i += 5
        if get_substring_as_int(bin_str, i, 1) == 0:
            break
        i = next_i

    consumed = next_i - start

    return res, next_i, consumed

def parse_sub_packet_with_length(bin_str, start, length):
    payloads = []
    next_i = start
    while length:
        payload, next_i, local_consumed = parse(bin_str, next_i)
        payloads.append(payload)
        length -= local_consumed

    consumed = next_i - start

    return payloads, next_i, consumed

def parse_sub_packet_with_count(bin_str, start, count):
    payloads = []
    next_i = start
    while count:
        payload, next_i, _ = parse(bin_str, next_i)
        payloads.append(payload)
        count -= 1

    consumed = next_i - start

    return payloads, next_i, consumed

def parse_operator(bin_str, start):
    if get_substring_as_int(bin_str, start, 1) == 0:
        sub_packet_length = get_substring_as_int(bin_str, start+1, 15)
        return parse_sub_packet_with_length(bin_str, start+1+15, sub_packet_length)
    else:
        sub_packet_count = get_substring_as_int(bin_str, start+1, 11)
        return parse_sub_packet_with_count(bin_str, start+1+11, sub_packet_count)

def parse(bin_str, start):
    i = start
    next_i = start

    version_number = get_substring_as_int(bin_str, i, 3)
    type_id = get_substring_as_int(bin_str, i+3, 3)

    packet = Packet(
        version_number=version_number,
        type_id=type_id,
    )

    if type_id == 4:
        value, next_i, _ = parse_literal(bin_str, i+6)
        packet.value = value
    else:
        local_subpackets, next_i, _ = parse_operator(bin_str, i+6)
        packet.subpackets += local_subpackets

    consumed = next_i - start

    return packet, next_i, consumed

if __name__ == "__main__":
    with open('input.txt', 'r') as f:
        hex_str = f.read().strip()

    bin_str = hex_str_to_bin_str(hex_str)

    outer_packet, _, _ = parse(bin_str, 0)
