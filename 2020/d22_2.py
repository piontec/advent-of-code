from typing import List, Tuple, Dict

# very slow, could use optimization, needs a few minutes to run
game_no = 0
game_cache: Dict[Tuple[Tuple[int], Tuple[int]], bool] = {}


def play_round(p1: List[int], p2: List[int]):
    card_1 = p1.pop(0)
    card_2 = p2.pop(0)
    if len(p1) >= card_1 and len(p2) >= card_2:
        sub_game_wins_p1 = play_game(p1[:card_1], p2[:card_2])
        target = p1 if sub_game_wins_p1 else p2
        last, first = (card_2, card_1) if sub_game_wins_p1 else (card_1, card_2)
    else:
        target = p1 if card_1 > card_2 else p2
        last, first = (card_1, card_2) if card_1 < card_2 else (card_2, card_1)
    target.append(first)
    target.append(last)


def play_game(p1: List[int], p2: List[int]) -> bool:
    """
    :param p1:
    :param p2:
    :return: True of player 1 wins, else False
    """
    global game_no
    global game_cache

    game_input = (tuple(p1.copy()), tuple(p2.copy()))

    game_no += 1
    if game_input in game_cache.keys():
        return game_cache[game_input]
    rounds: List[Tuple[List[int], List[int]]] = [(p1.copy(), p2.copy())]
    while len(p1) > 0 and len(p2) > 0:
        play_round(p1, p2)
        if any(prev_round == (p1, p2) for prev_round in rounds):
            return True
        else:
            rounds.append((p1.copy(), p2.copy()))
    first_wins = len(p1) > len(p2)
    game_cache[game_input] = first_wins
    return first_wins


def run_sim(lines: List[str]) -> int:
    try:
        empty_line_ind = lines.index("")
    except ValueError:
        empty_line_ind = lines.index("\n")
    hand_p1 = [int(x.strip()) for x in lines[1:empty_line_ind]]
    hand_p2 = [int(x.strip()) for x in lines[empty_line_ind + 2:]]

    play_game(hand_p1, hand_p2)
    winning_hand = hand_p1 if len(hand_p1) > 0 else hand_p2
    score = 0
    multi = len(winning_hand)
    for card in winning_hand:
        score += card * multi
        multi -= 1
    return score


test1 = """Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10
"""
res1 = run_sim(test1.splitlines())
assert res1 == 291

with open("i22.txt", "r") as f:
    lines = f.readlines()
res = run_sim(lines)
print(res)
