from typing import List


def make_one(board: List[int], e1: int, e2: int) -> (int, int):
    r_sum = board[e1] + board[e2]
    new_r = [int(r) for r in list("{:d}".format(r_sum))]
    board.extend(new_r)
    ne1 = (e1 + 1 + board[e1]) % len(board)
    ne2 = (e2 + 1 + board[e2]) % len(board)
    return ne1, ne2


def make_recipes(how_many: int) -> List[int]:
    board = [3, 7]
    e1 = 0
    e2 = 1
    while len(board) < 10 + how_many:
        e1, e2 = make_one(board, e1, e2)
    return board[how_many:how_many+11]


def find_sublist(where: List[int], what: List[int], start: int = 0) -> (int, int):
    if len(where) < len(what):
        return -1, 0
    found = 0
    for i in range(start, len(where)-len(what)+1):
        eq = True
        for j in range(i, i+len(what)):
            if where[j] != what[j-i]:
                eq = False
                found = i
                break
        if eq:
            return i, i
    return -1, found


def find_recipes(what: str) -> int:
    board = [3, 7]
    e1 = 0
    e2 = 1    
    f = [int(x) for x in list(what)]    
    last = 0
    while True:
        e1, e2 = make_one(board, e1, e2)        
        ind, last = find_sublist(board, f, last)
        if ind >= 0:
            return ind


res = make_recipes(2018)
res_txt = "".join(str(x) for x in res)
assert res_txt, "5941429882"

# # part 1
# res = make_recipes(209231)
# res_txt = "".join(str(x) for x in res)
# print(res_txt)

# part 2
assert find_recipes("51589"), 9
assert find_recipes("01245"), 5
print(find_recipes("209231"))
