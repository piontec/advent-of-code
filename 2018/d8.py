from typing import List, Tuple

class Node():
    def __init__(self):
        self.metadata: List[int] = []
        self.succs: List[Node] = []

    def Sum(self) -> int:
        sub_sum = sum(c.Sum() for c in self.succs)
        return sum(self.metadata) + sub_sum

    def Value(self) -> int:
        if len(self.succs) == 0:
            return sum(self.metadata)
        value = 0
        for i in self.metadata:
            if i == 0 or i > len(self.succs):
                continue
            value += self.succs[i-1].Value()
        return value


def parse(input: List[int]) -> Tuple[Node, List[int]]:
    cur_node = Node()
    node_count = input[0]
    meta_count = input[1]
    rest = input[2:]
    for _ in range(node_count):
        node, rest = parse(rest)
        cur_node.succs.append(node)
    cur_node.metadata = rest[:meta_count]
    rest = rest[meta_count:]
    return cur_node, rest


with open("i8.txt") as f:
    to_parse = f.read().strip()   

# test case
# to_parse = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"

int_list = [int(i) for i in to_parse.split(" ")]
n, r = parse(int_list)

# part 1
print(n.Sum())

# part 2
print(n.Value())