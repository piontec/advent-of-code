from functools import cache


dist_freqs = {
    3: 1,
    4: 3,
    5: 6,
    6: 7,
    7: 6,
    8: 3,
    9: 1,
}

# points_p1: int, points_p2: int, pos_p1: int, pos_p2: int
State = tuple[int, int, int, int]


@cache
def get_wins(state: State, active: int) -> tuple[int, int]:
    if state[active] >= 21:
        return (1, 0) if active == 0 else (0, 1)
    total_p1, total_p2 = 0, 0
    for (p, f) in dist_freqs.items():
        new_pos = ((state[2 + active] - 1 + p) % 10) + 1
        new_points = state[0 + active] + new_pos

        new_state = (new_points, state[1], new_pos, state[3]) if active == 0 else (state[0], new_points, state[2], new_pos)

        p1_won, p2_won = get_wins(new_state, (active + 1) % 2)
        total_p1 += f * p1_won
        total_p2 += f * p2_won
    return total_p1, total_p2


def run2(pos1: int, pos2: int, size: int = 10, limit: int = 21) -> int:
    state = (0, 0, pos1, pos2)

    p1_won, p2_won = get_wins(state, active=0)
    # FIXME
    return max(p1_won, p2_won) // 27


def run(pos1: int, pos2: int, size: int = 10, limit: int = 21) -> int:
    games_in_progress: dict[State, int] = {(0, 0, pos1, pos2): 1}
    games_won = {1: 0, 2: 0}

    while len(games_in_progress):
        state, freq = games_in_progress.popitem()
        for (p1, f1) in dist_freqs.items():
            pos_p1 = ((state[2] - 1 + p1) % size) + 1
            points_p1 = state[0] + pos_p1

            for (p2, f2) in dist_freqs.items():
                pos_p2 = ((state[3] - 1 + p2) % size) + 1
                points_p2 = state[1] + pos_p2

                new_freq = f1 * f2
                if points_p1 >= limit:
                    games_won[1] += freq * new_freq
                elif points_p2 >= limit:
                    games_won[2] += freq * new_freq
                else:
                    new_state = (points_p1, points_p2, pos_p1, pos_p2)
                    if new_state not in games_in_progress:
                        games_in_progress[new_state] = new_freq
                    else:
                        games_in_progress[new_state] *= new_freq
    return max(games_won[1], games_won[2])


def main() -> None:
    res = run2(4, 6)
    print(res)


def test() -> None:
    res = run2(4, 8)
    assert res == 444356092776315


if __name__ == "__main__":
    test()
    main()
