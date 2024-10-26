# from: https://advent-of-code.xavd.id/writeups/2023/day/20/
# make mermaid graph from input

lines = []

input = open("d20.txt").read().splitlines()
for line in input:
    raw_source, raw_target = line.split(" -> ")
    source = raw_source[1:]
    targets = raw_target.split(", ")

    for t in targets:
        if raw_source == "broadcaster":
            lines.append(f"  {raw_source} --> {t}")
        if raw_source.startswith("%"):
            lines.append(f"  {source} --> {t}")
        if raw_source.startswith("&"):
            # conjunction modules will be diamonds
            lines.append(f"  {source}{{{source}}} --> {t}")

with open("d20-graph.mmd", "w") as f:
    f.write("flowchart TD\n")
    for line in lines:
        f.write(line + "\n")
