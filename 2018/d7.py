from typing import List, Dict
import string
import pprint


Successors = Dict[str, List[str]]

def get_succ(lines: List[str]) -> Successors:
    succ_dict = {}
    for line in lines:
        entries = line.split(" ")
        pre, succ = entries[1], entries[7]
        if pre not in succ_dict.keys():
            succ_dict[pre] = [succ]
        else:
            succ_dict[pre].append(succ)
    all_succ = set([e for sl in succ_dict.values() for e in sl])
    all_pred = set(succ_dict.keys())
    lasts = sorted(list(all_succ.difference(all_pred)))
    for l in lasts:
        succ_dict[l] = []
    return succ_dict


def get_ready(succ_dict: Successors):
    all_succ = set([e for sl in succ_dict.values() for e in sl])
    all_pred = set(succ_dict.keys())  
    return all_pred.difference(all_succ)


def part1(lines: List[str]):
    succ_dict = get_succ(lines)    

    while len(succ_dict) > 0:
        first_ready = sorted(list(get_ready(succ_dict)))[0]
        succ_dict.pop(first_ready)
        print(first_ready, end='')
    print("")


def get_work_time(letter: string, base: int) -> int:    
    l = list(string.ascii_uppercase)
    return base + l.index(letter) + 1


def part2(lines: List[str], workers=5, base_time=60):
    time = 0
    tasks = {}
    succ_dict = get_succ(lines)
    ready_workers = list(range(workers))
    closest_finish_time = 0

    while len(succ_dict) > 0 or len(tasks) > 0:           
        # schedule
        ready_tasks = get_ready(succ_dict)
        in_progress = set(tasks[key][0] for key in tasks.keys())
        ready_tasks = sorted(list(ready_tasks.difference(in_progress)))
        sched_count = min([len(ready_workers), len(ready_tasks)])
        for _ in range(sched_count):
            worker = ready_workers.pop(0)
            task = ready_tasks.pop(0)
            finish_time = time + get_work_time(task, base_time)
            tasks[worker] = (task, finish_time)            
        # wait
        closest_finish_time = min([t[1] for t in tasks.values()])
        time = closest_finish_time
        finished = list(filter(lambda e: tasks[e][1] == time, tasks.keys()))
        for key in finished:
            succ_dict.pop(tasks[key][0])
            ready_workers.append(key)
            tasks.pop(key)
    print(time)


with open("i7.txt") as f:
    lines: List[str] = f.readlines()
pp = pprint.PrettyPrinter(indent=4)

# lines = ["Step C must be finished before step A can begin.",
# "Step C must be finished before step F can begin.",
# "Step A must be finished before step B can begin.",
# "Step A must be finished before step D can begin.",
# "Step B must be finished before step E can begin.",
# "Step D must be finished before step E can begin.",
# "Step F must be finished before step E can begin."]

part1(lines)
part2(lines)