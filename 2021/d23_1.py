from copy import deepcopy
from dataclasses import dataclass
from functools import cache
from typing import Optional, Iterator
import nographs as nog


@dataclass
class Rooms:
    # a[0] - upper room, connecting to corridor
    # a[1] - lower room
    a: list[str]
    b: list[str]
    c: list[str]
    d: list[str]

    def __eq__(self, other) -> bool:
        if type(other) is not Rooms:
            raise ValueError("wrong type")
        return self.a == other.a and self.b == other.b and self.c == other.c and self.d == other.d

    def __hash__(self) -> int:
        return hash(self.a[0]) + hash(self.a[1]) + hash(self.b[0]) + hash(self.b[1]) + hash(self.c[0])\
               + hash(self.c[1]) + hash(self.d[0]) + hash(self.d[1])


rooms_to_corridor_index = {"a": 2, "b": 4, "c": 6, "d": 8}
step_cost = {"a": 1, "b": 10, "c": 100, "d": 1000}


@dataclass
class Burrow:
    corridor: list[str]
    rooms: Rooms
    last_move_cost: int

    def __init__(self, corridor: Optional[list[str]], rooms: Rooms) -> None:
        if not corridor:
            self.corridor = ["." for _ in range(11)]
        else:
            assert len(corridor) == 11
            self.corridor = corridor
        self.rooms = rooms
        self.last_move_cost = 0

    def copy(self) -> 'Burrow':
        res = Burrow(self.corridor.copy(),
                     Rooms(self.rooms.a.copy(), self.rooms.b.copy(), self.rooms.c.copy(), self.rooms.d.copy()))
        return res

    def __eq__(self, other) -> bool:
        if type(other) is not Burrow:
            raise ValueError("wrong type")
        return self.corridor == other.corridor and self.rooms == other.rooms

    def __hash__(self) -> int:
        return hash(self.rooms) + hash(tuple(self.corridor)) + self.last_move_cost

    def __str__(self) -> str:
        return "\n".join([
            "#" * 13,
            "#" + "".join(self.corridor) + "#",
            f"###{self.rooms.a[0]}#{self.rooms.b[0]}#{self.rooms.c[0]}#{self.rooms.d[0]}###",
            f"  #{self.rooms.a[1]}#{self.rooms.b[1]}#{self.rooms.c[1]}#{self.rooms.d[1]}#",
            "  " + "#" * 9])


@cache
def get_all_moves_from_corridor(burrow: Burrow) -> list[Burrow]:
    res: list[Burrow] = []
    for i in range(len(burrow.corridor)):
        if burrow.corridor[i] == ".":
            continue
        amp = burrow.corridor[i]
        if amp not in ["a", "b", "c", "d"]:
            raise ValueError(f"impossible: {amp}")
        room = getattr(burrow.rooms, amp)
        # if we can't enter our room
        if not (room[1] == "." or (room[0] == "." and room[1] == amp)):
            continue
        room_index = 1 if room[1] == "." else 0
        # if the corridor is blocked by another amp
        dst_index = rooms_to_corridor_index[amp]
        s, e = (i, dst_index) if i < dst_index else (dst_index, i)
        blocked = False
        for pos in range(s, e + 1):
            if pos != i and burrow.corridor[pos] != ".":
                blocked = True
                break
        if blocked:
            continue
        # compute burrow after move
        new_bur = deepcopy(burrow)
        new_bur.corridor[i] = "."
        getattr(new_bur.rooms, amp)[room_index] = amp
        steps = abs(rooms_to_corridor_index[amp] - i) + room_index + 1
        new_bur.last_move_cost = steps * step_cost[amp]
        res.append(new_bur)
    return res


@cache
def get_all_moves_from_rooms(burrow: Burrow) -> list[Burrow]:
    res: list[Burrow] = []
    for room_name in ["a", "b", "c", "d"]:
        for room_index in range(2):
            room = getattr(burrow.rooms, room_name)
            # not an amp
            if room[room_index] == ".":
                continue
            # already OK
            amp = room[room_index]
            if room_name == amp and (room_index == 1 or (room_index == 0 and room[1] == amp)):
                continue
            # blocked by amp at index 0 - can't leave
            if room_index == 1 and room[0] != ".":
                continue
            possible_corridor_indexes = []
            # go left
            for ci in range(rooms_to_corridor_index[room_name] - 1, -1, -1):
                if burrow.corridor[ci] != ".":
                    break
                if ci in rooms_to_corridor_index.values():
                    continue
                possible_corridor_indexes.append(ci)
            # go right
            for ci in range(rooms_to_corridor_index[room_name] + 1, len(burrow.corridor)):
                if burrow.corridor[ci] != ".":
                    break
                if ci in rooms_to_corridor_index.values():
                    continue
                possible_corridor_indexes.append(ci)
            for ci in possible_corridor_indexes:
                new_bur = deepcopy(burrow)
                new_bur.corridor[ci] = amp
                getattr(new_bur.rooms, room_name)[room_index] = "."
                steps = abs(rooms_to_corridor_index[room_name] - ci) + room_index + 1
                new_bur.last_move_cost = steps * step_cost[amp]
                res.append(new_bur)
    return res


def next_edges(burrow: Burrow, _) -> Iterator[tuple[Burrow, int]]:
    for b in get_all_moves_from_rooms(burrow):
        yield b, b.last_move_cost
    for b in get_all_moves_from_corridor(burrow):
        yield b, b.last_move_cost

def heuristic(burrow: Burrow) -> int:
    res = 0
    # from corridor
    for i in range(len(burrow.corridor)):
        if burrow.corridor[i] == ".":
            continue
        amp = burrow.corridor[i]
        room = getattr(burrow.rooms, amp)
        room_index = 1 if room[1] == "." else 0
        steps = abs(rooms_to_corridor_index[amp] - i) + room_index + 1
        res += steps * step_cost[amp]
    # from rooms
    for room_name in ["a", "b", "c", "d"]:
        for room_index in range(2):
            room = getattr(burrow.rooms, room_name)
            amp = room[room_index]
            # not an amp
            if amp == ".":
                continue
            steps = abs(rooms_to_corridor_index[room_name] - rooms_to_corridor_index[amp])
            res += steps * step_cost[amp]
    return res




@cache
def find_best_cost(burrow: Burrow) -> int:
    goal = Burrow(None, Rooms(["a", "a"], ["b", "b"], ["c", "c"], ["d", "d"]))
    traversal = nog.TraversalAStar(next_edges)
    traversal.start_from(heuristic, burrow).go_to(goal)
    return traversal.path_length

def run(lines: list[str]) -> int:
    row0 = lines[2].strip().split("#")
    row1 = lines[3].strip().split("#")
    rooms = Rooms([row0[3].lower(), row1[1].lower()], [row0[4].lower(), row1[2].lower()],
                  [row0[5].lower(), row1[3].lower()], [row0[6].lower(), row1[4].lower()])
    burrow = Burrow(None, rooms)

    return find_best_cost(burrow)


def main() -> None:
    with open("i23.txt", "r") as i:
        lines = i.readlines()
    res = run(lines)
    print(res)



def test() -> None:
    txt = """#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########"""
    res = run(txt.splitlines())
    assert res == 12521


if __name__ == "__main__":
    test()
    main()
