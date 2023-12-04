
def main() -> None:
    with open("i2.txt", "r") as i:
        lines = i.readlines()
    depth = 0
    forward = 0
    for line in lines:
        instr, num_str = line.split(" ")
        num = int(num_str)
        match instr:
            case "forward":
                forward += num
            case "up":
                depth -= num
            case "down":
                depth += num
            case _:
                raise Exception("WTF")
    print(depth * forward)


if __name__ == "__main__":
    main()
