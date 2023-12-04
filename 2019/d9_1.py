
from typing import List, Dict
from itertools import permutations
from enum import IntEnum


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


cpu = CPU("104,1125899906842624,99", [])
cpu.run()
assert cpu.outputs[-1] == int("1125899906842624")
prog = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99"
cpu = CPU(prog, [])
cpu.run()
res = ",".join([str(x) for x in cpu.outputs])
assert res == prog
with open("i9.txt") as f:
    lines = f.readlines()
cpu = CPU(lines[0], [1])
cpu.run()
assert len(cpu.outputs) == 1
print(str.format("Part 1: {}", cpu.outputs[-1]))

cpu = CPU(lines[0], [2])
cpu.run()
assert len(cpu.outputs) == 1
print(str.format("Part 2: {}", cpu.outputs[-1]))
