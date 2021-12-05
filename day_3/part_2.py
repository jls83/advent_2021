class Node:
    def __init__(self, char):
        self.char = char
        self.children = {}
        self.sub_items = 0

class Thing:
    def __init__(self):
        self.root = Node(None)

    def insert(self, item):
        d = len(item)
        current = self.root
        for char in item:
            current.sub_items += d
            node = current.children.get(char, None)
            if node is None:
                node = Node(char)
                current.children[char] = node
            current = node
            d -= 1

    def max_path(self):
        res = ''
        current = self.root
        while True:
            if not current.children:
                return int(res, 2)
            max_pair = max((node.sub_items, k) for k, node in current.children.items())
            max_key = max_pair[-1]
            res += max_key
            current = current.children[max_key]

    def min_path(self):
        res = ''
        current = self.root
        while True:
            if not current.children:
                return int(res, 2)
            min_pair = min((node.sub_items, k) for k, node in current.children.items())
            min_key = min_pair[-1]
            res += min_key
            current = current.children[min_key]

if __name__ == "__main__":
    with open('input.txt', 'r') as fi:
        foo = [i.strip() for i in fi.readlines()]
    t = Thing()
    for item in foo:
        t.insert(item)
    print(t.max_path() * t.min_path())
