# from: https://advent-of-code.xavd.id/writeups/2023/day/20/
# make mermaid graph from input

lines = []

with open("d25.dot", mode="w") as f:
    f.write("graph G\n{\n")
    input = open("d25.txt").read().splitlines()
    for line in input:
        raw_source, raw_target = line.split(":", 1)

        for t in raw_target.split(" "):
            if not t:
                continue
            f.write(raw_source + " -- " + t + ";\n")
    f.write("}\n")
