
def main() -> None:
    with open("i1.txt", "r") as i:
        lines = i.readlines()
    depths = [int(line) for line in lines]
    increases = 0
    for i in range(1, len(depths)):
        if depths[i] > depths[i - 1]:
            increases += 1
    print(increases)


if __name__ == "__main__":
    main()
