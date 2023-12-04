def main() -> None:
    with open("i1.txt", "r") as i:
        lines = i.readlines()
    depths = [int(line) for line in lines]
    increases = 0
    for i in range(1, len(depths) - 2):
        sum_current = sum(depths[i:i + 3])
        sum_prev = sum(depths[i - 1:i + 2])
        if sum_current > sum_prev:
            increases += 1
    print(increases)


if __name__ == "__main__":
    main()
