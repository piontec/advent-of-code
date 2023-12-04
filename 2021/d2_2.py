
def main() -> None:
    with open("i2.txt", "r") as i:
        lines = i.readlines()
    depth = 0
    forward = 0
    aim = 0
    for line in lines:
        instr, num_str = line.split(" ")
        num = int(num_str)
        match instr:
            case "forward":
                forward += num
                depth += aim * num
            case "up":
                aim -= num
            case "down":
                aim += num
            case _:
                raise Exception("WTF")
    print(depth * forward)


if __name__ == "__main__":
    main()
