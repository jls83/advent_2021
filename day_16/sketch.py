class Packet(object):
    def __init__(self, version_number, type_id, payload):
        self.version_number = version_number
        self.type_id = type_id
        self.payload = payload

def hex_str_to_bin_str(hex_str):
    return bin(int(hex_str, 16))[2:]

def parse_with_content_length(hex_str, content_length):



def parse(bin_str):
    if len(bin_str) < 11:
        return None, None

    version_number = int(bin_str[0:3], 2)
    type_id = int(bin_str[3:6], 2)

    if type_id == 4:
        res_str = ''
        end = None  # TODO: disusting.
        for i in range(6, len(bin_str), 5):
            res_str += bin_str[i+1:i+5]
            end = i+5
            if bin_str[i] == '0':
                break

        return int(res_str, 2), bin_str[end:]
    else:
        length_type_id = bin_str[7]
        content_length = None
        content_count = None
        if length_type_id == '0':
            content_length = int(bin_str[8:8+16], 2)
            sub_packets, remainder = parse_with_content_length(bin_str[8+16:], content_length)
        else:  # if length_type_id == '`'
            content_count = int(bin_str[8:8+12], 2)
            sub_packets, remainder = parse_with_content_count(bin_str[8+12:], content_count)

