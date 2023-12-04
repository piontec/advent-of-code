from typing import List, Dict


class Node():
    def __init__(self, name: str, parent: 'Node' = None):
        self.name = name
        self.parent = parent


def count_orbits(nodes: Dict[str, Node], name: str):
    dist = 0
    node = nodes[name]
    while node.parent != None:
        node = node.parent
        dist += 1
    return dist


def run(lines: List[str]) -> int:
    nodes: Dict[str, Node] = {}
    for line in lines:
        c, o = line.strip().split(")")
        c_node: Node = None
        if c in nodes:
            c_node = nodes[c]
        else:
            c_node = Node(c)
            nodes[c] = c_node
        if o in nodes:
            nodes[o].parent = c_node
        else:
            o_node = Node(o, c_node)
            nodes[o] = o_node

    total = 0
    for name in nodes:
        if name != "COM":
            assert nodes[name].parent != None
        total += count_orbits(nodes, name)
    return total


lines = ["COM)B",
         "B)C",
         "C)D",
         "D)E",
         "E)F",
         "B)G",
         "G)H",
         "D)I",
         "E)J",
         "J)K",
         "K)L"]
assert run(lines) == 42
with open("i6.txt") as f:
    lines = f.readlines()
print(str.format("part1: {}", run(lines)))
