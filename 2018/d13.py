from typing import List, Dict, Tuple


with open("i13.txt") as f:
    lines: List[str] = f.readlines()

# # test - part1
# lines = [
# "/->-\        ",
# "|   |  /----\\",
# "| /-+--+-\  |",
# "| | |  | v  |",
# "\-+-/  \-+--/",
# "  \------/   "]

# # test - part 2
# lines = [
# "/>-<\  ",
# "|   |  ",
# "| /<+-\\",
# "| | | v",
# "\>+</ |",
# "  |   ^",
# "  \<->/"]

class Pos:
    def __init__(self, x, y):
        self.x = x
        self.y = y

    def equal(self, p: 'Pos') -> bool:
        return self.x == p.x and self.y == p.y


class Cart:
    def __init__(self, pos: Pos, direction: str):
        self.pos = pos
        self.direction = direction
        self.next_maneuver = "l"

    def update_maneuver(self):
        if self.next_maneuver == "l":
            self.next_maneuver = "s"
        elif self.next_maneuver == "s":
            self.next_maneuver = "r"
        else:
            self.next_maneuver = "l"

    def move_left(self):
        self.pos.x -= 1

    def move_right(self):
        self.pos.x += 1

    def move_up(self):
        self.pos.y -= 1

    def move_down(self):
        self.pos.y += 1

    def turn_left(self):
        if self.direction == ">":
            self.direction = "^"
            self.move_up()
        elif self.direction == "^":
            self.direction = "<"
            self.move_left()
        elif self.direction == "<":
            self.direction = "v"
            self.move_down()
        elif self.direction == "v":
            self.direction = ">"
            self.move_right()

    def turn_right(self):
        if self.direction == ">":
            self.direction = "v"
            self.move_down()
        elif self.direction == "v":
            self.direction = "<"
            self.move_left()
        elif self.direction == "<":
            self.direction = "^"
            self.move_up()
        elif self.direction == "^":
            self.direction = ">"
            self.move_right()

    def cont(self):
        if self.direction == ">":
            self.move_right()
        elif self.direction == "v":
            self.move_down()
        elif self.direction == "<":
            self.move_left()
        elif self.direction == "^":
            self.move_up()
    
    def turn_crossroad(self):
        if self.next_maneuver == "l":
            self.turn_left()
            self.update_maneuver()
        elif self.next_maneuver == "s":
            self.cont()
            self.update_maneuver()
        elif self.next_maneuver == "r":
            self.turn_right()
            self.update_maneuver()


def detect_carts(m: List[List[str]]) -> List[Cart]:
    max_y = len(m)
    max_x = len(m[0])
    carts = [Cart(Pos(x, y), m[y][x]) for x in range(max_x) for y in range(max_y) if m[y][x] in ["<", ">", "v", "^"]]
    return carts


def print_map(m: List[List[str]], carts: List[Cart]):
    for y in range(len(m)):
        for x in range(len(m[y])):
            found_c = False
            for c in carts:
                if c.pos.equal(Pos(x, y)):
                    print(c.direction, end="")
                    found_c = True
                    break
            if not found_c:
                print(m[y][x], end="")
        print("")
    print()


def detect_crash(m: List[List[str]], carts: List[Cart], stop_on_first: bool) -> Tuple[Pos, int]:
    for cart in carts:
        m[cart.pos.y][cart.pos.x] = "-" if cart.direction in ["<", ">"] else "|"
    # print_map(m, carts)
    time = 0        
    while True:
        to_remove = []
        time += 1
        carts.sort(key=lambda c: (c.pos.y, c.pos.x))
        for c in carts:
            if c in to_remove:
                continue
            if c.direction == ">":
                if m[c.pos.y][c.pos.x] == "-":                
                    c.move_right()
                elif m[c.pos.y][c.pos.x] == "/":
                    c.turn_left()
                elif m[c.pos.y][c.pos.x] == "\\":
                    c.turn_right()
                elif m[c.pos.y][c.pos.x] == "+":
                    c.turn_crossroad()
            elif c.direction == "<":
                if m[c.pos.y][c.pos.x] == "-":                
                    c.move_left()
                elif m[c.pos.y][c.pos.x] == "/":
                    c.turn_left()
                elif m[c.pos.y][c.pos.x] == "\\":
                    c.turn_right()
                elif m[c.pos.y][c.pos.x] == "+":
                    c.turn_crossroad()
            elif c.direction == "^":
                if m[c.pos.y][c.pos.x] == "|":
                    c.move_up()
                elif m[c.pos.y][c.pos.x] == "/":
                    c.turn_right()
                elif m[c.pos.y][c.pos.x] == "\\":
                    c.turn_left()
                elif m[c.pos.y][c.pos.x] == "+":
                    c.turn_crossroad()
            elif c.direction == "v":
                if m[c.pos.y][c.pos.x] == "|":
                    c.move_down()
                elif m[c.pos.y][c.pos.x] == "/":
                    c.turn_right()
                elif m[c.pos.y][c.pos.x] == "\\":
                    c.turn_left()
                elif m[c.pos.y][c.pos.x] == "+":
                    c.turn_crossroad()
            for other in carts:
                if other == c:
                    continue
                if other.pos.equal(c.pos):
                    if stop_on_first:
                        return c.pos, time
                    to_remove.append(c)
                    to_remove.append(other)                    
                    break
        for c in to_remove:
            carts.remove(c)        
        # print_map(m, carts)
        if len(carts) == 1:
            return carts[0].pos, time                            


# part1
# m = [list(line) for line in lines]
# carts = detect_carts(m)
# pos, t = detect_crash(m, carts, True)
# print(pos.x, pos.y, t)

# part2
m = [list(line) for line in lines]
carts = detect_carts(m)
pos, t = detect_crash(m, carts, False)
print(pos.x, pos.y, t)