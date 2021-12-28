class Board:
    BOARD_WIDTH = 5
    BOARD_HEIGHT = 5

    ALL_ROW_VALS = set(range(BOARD_WIDTH))
    ALL_COL_VALS = set(range(BOARD_HEIGHT))

    def __init__(self):
        self._map = {}
        self._uncalled_numbers = set()
        self._called_rows = set()
        self._called_cols = set()

    @property
    def is_complete(self):
        return self._called_rows == self.ALL_ROW_VALS \
            or self._called_cols == self.ALL_COL_VALS

    def mark_number(self, n):
        if n not in self._map:
            return

        r, c = self._map[n]
        self._called_rows.add(r)
        self._called_cols.add(c)

        self._uncalled_numbers.remove(n)

    def get_sum_uncalled_numbers(self):
        return sum(self._uncalled_numbers)

    @classmethod
    def from_input(cls, coll):
        board = cls()
        for r, row in enumerate(coll):
            row_parsed = (int(n) for n in row.split(' ') if n != '')
            for c, val in enumerate(row_parsed):
                coord = (r, c)
                board._map[val] = coord
                board._uncalled_numbers.add(val)

        return board

def parse_input():
    with open('input.txt', 'r') as f:
        lines = [line.strip() for line in list(f)]
    
    numbers_raw, _, *boards_raw = lines
    
    numbers = [int(n) for n in numbers_raw.split(',')]

    boards_unparsed = []
    current_board = []
    for line in boards_raw:

        if line == '':
            boards_unparsed.append(current_board)
            current_board = []
            continue

        current_board.append(line)

    if current_board:
        boards_unparsed.append(current_board)

    return numbers, boards_unparsed

if __name__ == "__main__":
    numbers, boards_raw = parse_input()
    boards = [Board.from_input(b) for b in boards_raw]

    for n in numbers:
        completed_boards = []
        for b in boards:
            b.mark_number(n)
            if b.is_complete:
                completed_boards.append(b)
        if completed_boards:
            foo = min(b.get_sum_uncalled_numbers() for b in completed_boards)
            print(foo * n)
            break
