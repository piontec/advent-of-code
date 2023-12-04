from typing import List, Dict, Tuple
import collections, itertools


with open("i12.txt") as f:
    lines: List[str] = f.readlines()

# # test - results in 325
# lines = ["initial state: #..#.#..##......###...###",
# "",
# "...## => #",
# "..#.. => #",
# ".#... => #",
# ".#.#. => #",
# ".#.## => #",
# ".##.. => #",
# ".#### => #",
# "#.#.# => #",
# "#.### => #",
# "##.#. => #",
# "##.## => #",
# "###.. => #",
# "###.# => #",
# "####. => #"]

def extend_state(state: collections.deque) -> Tuple[int, collections.deque]:
    found = False
    ind = 0
    offset = 0
    for i in range(5):
        if state[i] == "#":
            found = True
            ind = i
            break
    if found:
        offset = 5-ind        
        state.extendleft("." for _ in range(offset))
    found = False
    ind = 0
    sl = len(state)
    for i in range(sl - 1, sl - 6, -1):
        if state[i] == "#":
            found = True
            ind = i
            break
    if found:
        state.extend("." for _ in range(5-(sl-ind-1)))
    return offset, state


def get_result(pattern: List[str], rules: Dict[str, str]) -> str:
    p = "".join(pattern)
    if p in rules.keys():
        return rules[p]
    return "."


def get_state(i, state, rules):
    pattern = list(itertools.islice(state, i-2, i+3))
    return get_result(pattern, rules)


start = lines[0].split(":")[1].strip()
state = collections.deque(list(start))
rules = {l[0:5]:l[9] for l in lines[2:]}

# # part 1
# iters = 20
# offset = 0
# for i in range(1, iters+1): 
#     o, state = extend_state(state)
#     offset -= o
#     new_state = state.copy()
#     print("{:3d} ".format(i), end='')
#     for i in range(3, len(state)-3):
#         new_state[i] = get_state(i, state, rules)
#         print(new_state[i], end='')    
#     print("")
#     state = new_state
# res = sum(0 if state[i] == "." else i+offset for i in range(len(state)))
# print(res)
# #end part1
    
# part 2
iters = 95
offset = 0
for i in range(1, iters+1): 
    o, state = extend_state(state)
    offset -= o
    new_state = state.copy()
    # print("{:3d} ".format(i), end='')
    for i in range(3, len(state)-3):
        new_state[i] = get_state(i, state, rules)
        # print(new_state[i], end='')    
    # print("")
    state = new_state

res = sum(0 if state[i] == "." else i+offset for i in range(len(state)))
plant_count = sum(0 if state[i] == "." else 1 for i in range(len(state)))
total = res + (50000000000-95) * plant_count
print(total)
#end part2