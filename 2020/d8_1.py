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

    def run_until_loop_detected(self, instructions: List[str]) -> None:
        executed_instruction_indices = []
        while True:
            instruction = instructions[self._ip].strip()
            executed_instruction_indices.append(self._ip)
            self.execute_instruction(instruction)
            if self._ip in executed_instruction_indices:
                return


test1 = """nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"""

c1 = Cpu()
c1.run_until_loop_detected(test1.splitlines())
assert c1.accumulator == 5

with open("i8.txt", "r") as f:
    lines = f.readlines()

c2 = Cpu()
c2.run_until_loop_detected(lines)
print(c2.accumulator)
