from typing import List

class Node:
    def __init__(self, val: int):
        self.val = val
        self.next = None
        self.prev = None


def highscore(players, last: int) -> int:
    first = Node(0)
    first.next = first
    first.prev = first
    scores = {i:0 for i in range(players)}
    current_player = 0
    current_marble = first
    for i in range(1, last + 1):
        if i % 23 == 0:
            to_remove = current_marble
            for _ in range(7):
                to_remove = to_remove.prev            
            points = i
            points += to_remove.val
            scores[current_player] += points
            prev_node = to_remove.prev
            next_node = to_remove.next
            prev_node.next = next_node
            next_node.prev = prev_node
            current_marble = next_node
        else:
            next_node = current_marble.next
            next_next_node = next_node.next 
            new_node = Node(i)
            next_node.next = new_node
            new_node.prev = next_node
            new_node.next = next_next_node
            next_next_node.prev = new_node
            current_marble = new_node
        current_player = (current_player + 1) % players
    return max(scores.values())

# test cases
assert highscore(9, 25)==32
assert highscore(10, 1618)==8317
assert highscore(13, 7999)==146373
assert highscore(17, 1104)==2764
assert highscore(21, 6111)==54718
assert highscore(30, 5807)==37305

# part1
print(highscore(465, 71940))

# # part2
print(highscore(465, 7194000))