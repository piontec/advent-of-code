class Die:
    def __init__(self):
        self._val = 0
        self._rolls = 0

    def roll(self, times: int = 3) -> int:
        res = 0
        for i in range(times):
            self._val = (self._val % 100) + 1
            self._rolls += 1
            res += self._val
        return res

    @property
    def rolls(self) -> int:
        return self._rolls


def main() -> None:
    res = run(4, 6)
    print(res)


def run(p1_pos: int, p2_pos: int, size: int = 10) -> int:
    die = Die()
    scores = [0, 0]
    positions = [p1_pos, p2_pos]
    win_ind = -1
    while True:
        for i in [0, 1]:
            positions[i] = ((positions[i] - 1 + die.roll()) % size) + 1
            scores[i] += positions[i]
            if scores[i] >= 1000:
                win_ind = i
                break
        if win_ind != -1:
            break
    looser_score = (scores[(win_ind + 1) % 2])
    rolls = die.rolls
    return looser_score * rolls


def test() -> None:
    res = run(4, 8)
    assert res == 739785


if __name__ == "__main__":
    test()
    main()
