import copy
from collections import deque
from dataclasses import dataclass

Graph = dict[str, list[str]]


@dataclass
class Path:
    path: list[str]
    has_double: bool = False


def main() -> None:
    with open("i12.txt", "r") as i:
        lines = i.readlines()
    res = run(lines)
    print(res)


def add_to_graph(graph: Graph, n1: str, n2: str):
    if n1 not in graph:
        graph[n1] = [n2]
    else:
        graph[n1].append(n2)


def run(lines: list[str]) -> int:
    graph: Graph = {}
    for line in lines:
        node1, node2 = line.strip().split("-")
        add_to_graph(graph, node1, node2)
        if not (node2 == "end" or node1 == "start"):
            add_to_graph(graph, node2, node1)
    paths: list[Path] = []
    partials = deque([Path(["start"])])
    while len(partials) > 0:
        partial = partials.pop()
        last = partial.path[-1]
        for n in graph[last]:
            if n == "start":
                continue
            set_double = False
            if n.islower() and n in partial.path:
                if partial.has_double:
                    continue  # stop processing this partial
                else:
                    set_double = True
            new_partial = copy.deepcopy(partial)
            new_partial.path.append(n)
            if set_double:
                new_partial.has_double = set_double
            if n == "end":
                paths.append(new_partial)
            else:
                partials.append(new_partial)
    return len(paths)


def test() -> None:
    txt = """start-A
start-b
A-c
A-b
b-d
A-end
b-end"""
    assert run(txt.splitlines()) == 36

    txt = """dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc"""
    assert run(txt.splitlines()) == 103


if __name__ == "__main__":
    test()
    main()
