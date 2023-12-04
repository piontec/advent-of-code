from typing import List, Dict


class Node():
    def __init__(self, name: str, parent: 'Node' = None):
        self.name = name
        self.parent = parent


def get_path_to_root(nodes: Dict[str, Node], name: str) -> List[Node]:
    path: List[Node] = []
    node = nodes[name]
    while node.parent != None:
        node = node.parent
        path.append(node)
    return path


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

    mine = get_path_to_root(nodes, "YOU")
    santas = get_path_to_root(nodes, "SAN")
    length = 0
    common: Node = None
    for node in mine:
        if node in santas:
            common = node
            break
        length += 1
    assert common != None
    for node in santas:
        if node == common:
            break
        length += 1
    return length


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
         "K)L",
         "K)YOU",
         "I)SAN"]
assert run(lines) == 4
with open("i6.txt") as f:
    lines = f.readlines()
print(str.format("part2: {}", run(lines)))
