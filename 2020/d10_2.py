from dataclasses import dataclass
from typing import List, Dict


@dataclass
class Node:
    val: int
    succ: List["Node"]
    path_count = 0


def get_or_create_node(node_map: Dict[int, Node], val: int) -> Node:
    if val in node_map.keys():
        return node_map[val]
    node = Node(val=val, succ=[])
    node_map[val] = node
    return node


def build_graph(num_lines: List[int]) -> Dict[int, Node]:
    node_map: Dict[int, Node] = {}
    for i in range(len(num_lines)):
        node = get_or_create_node(node_map, num_lines[i])
        for j in range(i + 1, len(num_lines)):
            diff = num_lines[j] - num_lines[i]
            if diff <= 3:
                node.succ.append(get_or_create_node(node_map, num_lines[j]))
            else:
                break
    return node_map


def count_paths(lines: List[str]) -> int:
    num_lines = [int(x.strip()) for x in lines]
    num_lines.append(0)
    num_lines.append(max(num_lines) + 3)
    num_lines.sort()
    node_map = build_graph(num_lines)
    num_lines.reverse()
    for num in num_lines:
        node = node_map[num]
        if len(node.succ) == 0:
            node.path_count = 1
            continue
        node.path_count = sum(n.path_count for n in node.succ)
    return node_map[0].path_count


test1 = """16
10
15
5
1
11
7
19
6
12
4
"""

res = count_paths(test1.splitlines())
assert res == 8

test2 = """28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3
"""

res = count_paths(test2.splitlines())
assert res == 19208

with open("i10.txt", "r") as f:
    lines = f.readlines()
res = count_paths(lines)
print(res)
