import sys
from datetime import datetime


with open("i4.txt") as f:
    lines = f.readlines()

entries = []
for line in lines:
    fields = line.split("]")
    date = datetime.strptime(fields[0][1:], "%Y-%m-%d %H:%M")
    entries.append((date, fields[1]))

times = {}
gid = 0
se = sorted(entries, key=lambda e: e[0])
start = None
for e in se:
    log = e[1].strip()
    if log.startswith("Guard"):
        gid = int(log.split(" ")[1][1:])
        if gid not in times.keys():
            times[gid] = [0] * 60
        continue
    if log.startswith("falls asleep"):
        start = e[0]
        continue
    # must be another entry - "wakes up"
    for i in range(start.minute, e[0].minute):
        times[gid][i]+=1

# part 1
total = {gid:sum(v) for (gid, v) in times.items()}
max_gid = max(total, key=lambda e: total[e])
print(max_gid)
max_min = max(times[max_gid])
max_min_ind = times[max_gid].index(max_min)
print(max_min_ind)
print("* {}".format(max_min_ind * max_gid))

# part 2
maxes = {gid:max(v) for (gid, v) in times.items()}
max_gid = max(maxes, key=lambda e: maxes[e])
print(max_gid)
max_min = times[max_gid].index(maxes[max_gid])
print(max_min)
print("* {}".format(max_min * max_gid))
