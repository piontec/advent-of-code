
from typing import List, Dict, Tuple
from itertools import permutations
from enum import IntEnum


Point = Tuple[int, int]
Data = Tuple[int, int]


class StopType(IntEnum):
    STOP = 0
    OUTPUT = 1


class CPU:
    def __init__(self, strcode: str, inputs: List[int]):
        super().__init__()
        self._ip: int = 0
        self._rb: int = 0
        self._input_index = 0
        self.outputs: List[int] = []
        self.inputs: List[int] = inputs
        self._code: Dict[int, int] = {}
        strcodes = str.split(strcode, ",")
        for i in range(len(strcodes)):
            self._code[i] = int(strcodes[i])

    def _parse_instruction(self) -> (int, int, List[int]):
        instruction = self._code[self._ip]
        opcode = instruction % 100
        no_args = 0
        if opcode == 1 or opcode == 2 or opcode == 7 or opcode == 8:
            no_args = 3
        elif opcode == 3 or opcode == 4 or opcode == 9:
            no_args = 1
        elif opcode == 5 or opcode == 6:
            no_args = 2
        else:
            assert False
        modes: List[int] = []
        mds = instruction // 100
        for i in range(no_args):
            mode = mds % 10
            mds //= 10
            modes.append(mode)
        return (opcode, no_args, modes)

    def _get_argument(self, arg_modes: List[int], offset: int) -> int:
        # position mode
        if arg_modes[offset] == 0:
            addr = self._code[self._ip + 1 + offset]
            if addr not in self._code.keys():
                self._code[addr] = 0
            return self._code[addr]
        # immediate mode
        if arg_modes[offset] == 1:
            return self._code[self._ip + 1 + offset]
        # relative mode
        if arg_modes[offset] == 2:
            addr = self._code[self._ip + 1 + offset] + self._rb
            if addr not in self._code.keys():
                self._code[addr] = 0
            return self._code[addr]
        raise AssertionError(str.format(
            "Unknown arg mode: {}", arg_modes[offset]))

    def run(self, stop: StopType = StopType.STOP) -> StopType:
        outputSet = False
        while not(self._code[self._ip] == 99 or (stop == StopType.OUTPUT and outputSet)):
            opcode, no_args, modes = self._parse_instruction()
            if opcode == 1 or opcode == 2:
                assert modes[2] == 0 or modes[2] == 2
                v1 = self._get_argument(modes, 0)
                v2 = self._get_argument(modes, 1)
                rb_offset = 0 if modes[2] == 0 else self._rb
                self._code[self._code[self._ip + 3] + rb_offset] = v1 + \
                    v2 if opcode == 1 else v1 * v2
            elif opcode == 3:
                assert modes[0] == 0 or modes[0] == 2
                rb_offset = 0 if modes[0] == 0 else self._rb
                self._code[self._code[self._ip + 1] +
                           rb_offset] = self.inputs[self._input_index]
                self._input_index += 1
            elif opcode == 4:
                if modes[0] == 0:
                    self.outputs.append(self._code[self._code[self._ip + 1]])
                elif modes[0] == 1:
                    self.outputs.append(self._code[self._ip + 1])
                elif modes[0] == 2:
                    self.outputs.append(
                        self._code[self._code[self._ip + 1] + self._rb])
                else:
                    assert False
                outputSet = True
            elif opcode == 5 or opcode == 6:
                val = self._get_argument(modes, 0)
                if (opcode == 5 and val != 0) or (opcode == 6 and val == 0):
                    self._ip = self._get_argument(modes, 1)
                    continue
            elif opcode == 7 or opcode == 8:
                assert modes[2] == 0 or modes[2] == 2
                v1 = self._get_argument(modes, 0)
                v2 = self._get_argument(modes, 1)
                res = (opcode == 7 and v1 < v2) or (opcode == 8 and v1 == v2)
                rb_offset = 0 if modes[2] == 0 else self._rb
                self._code[self._code[self._ip + 3] + rb_offset] = int(res)
            elif opcode == 9:
                val = self._get_argument(modes, 0)
                self._rb += val
            else:
                print(str.format("ERROR: unknown op code: {}",
                                 self._code[self._ip]))
                return
            self._ip += no_args + 1
        if stop == StopType.STOP:
            return StopType.STOP
        return StopType.OUTPUT if outputSet else StopType.STOP


class Robot:
    class StepResult:
        def __init__(self, pos: Point, color: int, stop_type: StopType):
            self.pos = pos
            self.color = color
            self.stop_type = stop_type

    def __init__(self, strcode: str):
        super().__init__()
        self._pos: Point = (0, 0)
        # 0 - up; 1 - right; 2 - down; 3 - left
        self._dir = 0
        self._cpu = CPU(strcode, [])

    def AddInput(self, val: int):
        self._cpu.inputs.append(val)

    def GetPos(self) -> Point:
        return self._pos

    def RunStep(self) -> StepResult:
        stop_type = self._cpu.run(stop=StopType.OUTPUT)
        if stop_type == StopType.STOP:
            return Robot.StepResult(self._pos, -1, StopType.STOP)
        # we need to get 2 outputs; if we got 1st, we should get 2nd
        self._cpu.run(stop=StopType.OUTPUT)
        color = self._cpu.outputs[-2]
        dir = self._cpu.outputs[-1]
        self._move(dir)
        return Robot.StepResult(self._pos, color, StopType.OUTPUT)

    def _move(self, dir: int):
        # turn left
        if dir == 0:
            self._dir = (self._dir - 1) % 4
        # turn right
        elif dir == 1:
            self._dir = (self._dir + 1) % 4
        else:
            assert False

        if self._dir == 0:
            self._pos = (self._pos[0], self._pos[1] + 1)
        elif self._dir == 1:
            self._pos = (self._pos[0] + 1, self._pos[1])
        elif self._dir == 2:
            self._pos = (self._pos[0], self._pos[1] - 1)
        elif self._dir == 3:
            self._pos = (self._pos[0] - 1, self._pos[1])


class Hull:
    def __init__(self, strcode: str, starting_color: int):
        super().__init__()
        self._hull: Dict[Point, Data] = {(0, 0): (starting_color, 0)}
        self._robot = Robot(strcode)

    def run(self):
        while True:
            pos = self._robot.GetPos()
            if pos not in self._hull:
                self._hull[pos] = (0, 0)
            color = self._hull[pos][0]
            self._robot.AddInput(color)
            res = self._robot.RunStep()
            if res.stop_type == StopType.STOP:
                break
            new_val = (res.color, self._hull[pos][1] + 1)
            self._hull[pos] = new_val

    def count_panels(self):
        painted = [1 for p in self._hull if self._hull[p][1] > 0]
        return sum(painted)

    def print(self):
        min_x = min_y = max_x = max_y = 0
        for pos in self._hull:
            if pos[0] < min_x:
                min_x = pos[0]
            if pos[0] > max_x:
                max_x = pos[0]
            if pos[1] < min_y:
                min_y = pos[1]
            if pos[1] > max_y:
                max_y = pos[1]
            
        for y in range(max_y, min_y - 1, -1):
            for x in range(min_x, max_x + 1):
                pos = (x, y)
                if pos not in self._hull:
                    print(" ", end='')
                    continue
                if self._hull[pos][0] == 0:
                    print(" ", end='')
                else:
                    print("#", end='')
            print("")


with open("i11.txt") as f:
    lines = f.readlines()

h = Hull(lines[0], 0)
h.run()
print("Part 1: {}".format(h.count_panels()))

h = Hull(lines[0], 1)
h.run()
print("Part 2")
h.print()
