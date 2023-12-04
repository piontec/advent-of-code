import copy
from collections import deque

Graph = dict[str, list[str]]


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
    paths: list[list[str]] = []
    partials = deque([["start"]])
    while len(partials) > 0:
        partial = partials.pop()
        last = partial[-1]
        for n in graph[last]:
            if n.islower() and n in partial:
                continue
            path = copy.deepcopy(partial)
            path.append(n)
            if n == "end":
                paths.append(path)
            else:
                partials.append(path)
    return len(paths)


def test() -> None:
    txt = """start-A
start-b
A-c
A-b
b-d
A-end
b-end"""
    assert run(txt.splitlines()) == 10


if __name__ == "__main__":
    test()
    main()
