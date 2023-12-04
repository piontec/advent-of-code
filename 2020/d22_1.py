from typing import List


def play_round(p1: List[int], p2: List[int]) -> None:
    card_1 = p1.pop(0)
    card_2 = p2.pop(0)
    target = p1 if card_1 > card_2 else p2
    low, high = (card_1, card_2) if card_1 < card_2 else (card_2, card_1)
    target.append(high)
    target.append(low)


def run_sim(lines: List[str]) -> int:
    try:
        empty_line_ind = lines.index("")
    except ValueError:
        empty_line_ind = lines.index("\n")
    hand_p1 = [int(x.strip()) for x in lines[1:empty_line_ind]]
    hand_p2 = [int(x.strip()) for x in lines[empty_line_ind + 2:]]

    while len(hand_p1) > 0 and len(hand_p2) > 0:
        play_round(hand_p1, hand_p2)
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
assert res1 == 306

with open("i22.txt", "r") as f:
    lines = f.readlines()
res = run_sim(lines)
print(res)
