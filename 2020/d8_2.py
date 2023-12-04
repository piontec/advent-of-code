from typing import List


class Cpu:

    def __init__(self):
        self._ip = 0
        self._acc = 0

    @property
    def accumulator(self) -> int:
        return self._acc

    def execute_instruction(self, instr: str) -> None:
        op_code, op_arg_txt = instr.split()
        op_arg = int(op_arg_txt)
        if op_code == "nop":
            pass
        elif op_code == "jmp":
            self._ip += op_arg
            return
        elif op_code == "acc":
            self._acc += op_arg
        else:
            raise Exception(f"unknown instruction {op_code}")
        self._ip += 1

    def run_until_loop_detected(self, instructions: List[str]) -> bool:
        executed_instruction_indices = []
        while True:
            instruction = instructions[self._ip].strip()
            executed_instruction_indices.append(self._ip)
            self.execute_instruction(instruction)
            if self._ip < 0 or self._ip > len(instructions):
                raise Exception(f"Impossible InstructionPointer {self._ip}")
            if self._ip in executed_instruction_indices:
                return True
            if self._ip == len(instructions):
                return False


def fix_instructions(instructions: List[str]) -> Cpu:
    for instr_ind in range(len(instructions)):
        instr_copy = instructions.copy()
        instr = instr_copy[instr_ind]
        if instr.startswith("nop"):
            instr = instr.replace("nop", "jmp")
        elif instr.startswith("jmp"):
            instr = instr.replace("jmp", "nop")
        instr_copy[instr_ind] = instr
        cpu = Cpu()
        if not cpu.run_until_loop_detected(instr_copy):
            return cpu
    raise Exception("No program completed without detecting loop")


test1 = """nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"""

c1 = fix_instructions(test1.splitlines())
assert c1.accumulator == 8

with open("i8.txt", "r") as f:
    lines = f.readlines()

c2 = fix_instructions(lines)
print(c2.accumulator)
