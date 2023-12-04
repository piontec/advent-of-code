from typing import List


class ALUInterpreter:
    def __init__(self, code_lines: List[str], inputs: List[int]):
        self._ip: int = 0
        self._input_index = 0
        self._code_lines = code_lines
        self._inputs = inputs
        self.regs = {
            "w": 0,
            "x": 0,
            "y": 0,
            "z": 0,
        }

    def _run_instruction(self) -> None:
        instruction = self._code_lines[self._ip].strip('\n')
        self._ip += 1
        us = instruction.split(" ")
        op = us[0]
        args = us[1:]
        if op == "inp":
            self.regs[args[0]] = self._inputs[self._input_index]
            self._input_index += 1
        else:
            val = self.regs[args[1]] if args[1] in self.regs.keys() else int(args[1])
            if op == "add":
                self.regs[args[0]] += val
            elif op == "mul":
                self.regs[args[0]] *= val
            elif op == "div":
                if val == 0:
                    raise ZeroDivisionError()
                self.regs[args[0]] //= val
            elif op == "mod":
                if val <= 0:
                    raise ValueError(f"wrong b arg of mod: {val}")
                if self.regs[args[0]] < 0:
                    raise ValueError(f"wrong a arg of mod: {val}")
                self.regs[args[0]] %= val
            elif op == "eql":
                self.regs[args[0]] = int(self.regs[args[0]] == val)
            else:
                raise ValueError(f"Wrong op: {op}")

    def run(self) -> None:
        while self._ip < len(self._code_lines):
            self._run_instruction()

    def __str__(self) -> str:
        return f"w:{self.regs['w']}, x:{self.regs['x']}, y:{self.regs['y']}, z:{self.regs['z']},"


class ALU:
    def __init__(self, inputs: List[int]):
        self._ip: int = 0
        self._input_index = 0
        self._inputs = inputs
        self.regs = {
            "w": 0,
            "x": 0,
            "y": 0,
            "z": 0,
        }

    def __str__(self) -> str:
        return f"x:{self.regs['x']}, y:{self.regs['y']}, z:{self.regs['z']},"
        # return f"w:{self.regs['w']}, x:{self.regs['x']}, y:{self.regs['y']}, z:{self.regs['z']},"

    def run_one_digit(self, w: int, div_z: bool, add_x: int, add_y: int, print_it: bool = False):
        self.regs["x"] = self.regs["z"] % 26
        if div_z:
            self.regs["z"] //= 26

        if div_z and self.regs["x"] + add_x == w:
            # self.regs["x"] = 0
            self.regs["y"] = 0
        else:
            # self.regs["x"] = 1
            self.regs["y"] = w + add_y
            self.regs["z"] = self.regs["z"] * 26 + self.regs["y"]
        if print_it:
            print(f"input={w}, div_z={div_z}, add_x={add_x}, add_y={add_y}, y_push={self.regs['y']}, {self}")

    def run(self, print_it: bool = False) -> None:
        runs = [
            (False, 14, 16),
            (False, 11, 3),
            (False, 12, 2),
            (False, 11, 7),
            (True, -10, 13),
            (False, 15, 6),
            (True, -14, 10),
            (False, 10, 11),
            (True, -4, 6),
            (True, -3, 5),
            (False, 13, 11),
            (True, -3, 4),
            (True, -9, 4),
            (True, -12, 6)
        ]
        for i in range(14):
            div_z, add_x, add_y = runs[i]
            self.run_one_digit(self._inputs[i], div_z, add_x, add_y, print_it)


def get_lower(val: int) -> int:
    val_str = str(val)
    if "0" in val_str:
        ind = val_str.find("0")
        sub = int(val_str[ind:]) + 1
        val -= sub
    return val


def main() -> None:
    with open("i24.txt") as f:
        lines = f.readlines()
    high = 99999999999999
    high = 99985912989614
    while True:
        high = get_lower(high)
        inputs = [int(c) for c in str(high)]
        alu = ALU(inputs)
        alu.run()
        if alu.regs["z"] == 0:
            print(high)
            break
        high -= 1


def test() -> None:
    with open("i24.txt") as f:
        lines = f.readlines()
    num = 59985912981939
    inputs = [int(c) for c in str(num)]
    a = ALU(inputs)
    a.run(print_it=True)
    ai = ALUInterpreter(lines, inputs)
    ai.run()

    assert ai.regs["z"] == a.regs["z"]

# notepad

# i1 = i14 - 4
# i2 = i13 + 6
# i3 = i10 + 1
# i4 = i5 + 3
# i6 = i7 + 8
# i8 = i9 - 7
# i11 = i12 - 8
# max: 59996912981939
# min: 17241911811915

def run_one_digit(w: int, div_z: bool, add_x: int, add_y: int):
    runs = {
        (False, 14, 16),
        (False, 11, 3),
        (False, 12, 2),
        (False, 11, 7),
        (True, -10, 13),
        (False, 15, 6),
        (True, -14, 10),
        (False, 10, 11),
        (True, -4, 6),
        (True, -3, 5),
        (False, 13, 11),
        (True, -3, 4),
        (True, -9, 4),
        (True, -12, 6)
    }
    global z
    """
w = w
x = 0
x += z
x %= 26
z = z // 26 if div_z else z
x += add_x
x = x == w
x = x == 0
y = 0
y += 25
y *= x
y += 1
z *= y
y = 0
y += w
y += add_y
y *= x
z += y
    """

    x = z % 26
    z = z // 26 if div_z else z  # when div_z == False, then add_x > 10, else add_x < 0
    x += add_x
    x = x == w  # always False when div_z == False
    x = x == 0  # always True when div_z == False => x = 1
    y = 25
    y *= x
    y += 1
    z *= y
    y = w
    y += add_y
    y *= x
    z += y

def run_one_digit2(w: int, div_z: bool, add_x: int, add_y: int):
    global z

    x = z % 26
    if div_z:
        z //= 26
        x = x + add_x != w  # 0 or 1
        y = (25 * x) + 1  # 1 or 26
        z *= y
        y = (w + add_y) * x
        z += y
    else:
        x = 1
        y = 26
        z = z * 26
        y = (w + add_y) * x
        z = z + y

def run_one_digit3(w: int, div_z: bool, add_x: int, add_y: int):
    global z

    x = z % 26
    if div_z:
        z //= 26
        if x + add_x != w:
            x = 1
            y = w + add_y
            z = z * 26 + y
        else:
            x = 0
            y = 0
    else:
        x = 1
        y = w + add_y
        z = z * 26 + y

def run_one_digit4(w: int, div_z: bool, add_x: int, add_y: int):
    global z

    x = z % 26
    if div_z:
        z //= 26

    if div_z and x + add_x == w:
        x = 0
        y = 0
    else:
        x = 1
        y = w + add_y
        z = z * 26 + y




if __name__ == "__main__":
    test()
    main()
