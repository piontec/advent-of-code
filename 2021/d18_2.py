def main() -> None:
    with open("i18.txt", "r") as i:
        lines = i.readlines()
    res = run(lines)
    print(res)


def reduce_one(expr: str) -> str:
    while True:
        no_explode = True
        no_split = True

        bracket_level = 0
        pair_start_index = 0
        pair_end_index = 0
        for i in range(len(expr)):
            c = expr[i]
            if c == "[":
                bracket_level += 1
                pair_start_index = i
            elif c == "]":
                bracket_level -= 1
            elif c != ",":
                pair_end_index = pair_start_index + 1
                while expr[pair_end_index] != "]":
                    pair_end_index += 1
                    if expr[pair_end_index] == "[":
                        pair_end_index = pair_start_index
                        break
                if bracket_level > 4 and pair_end_index > pair_start_index:
                    expr = explode(expr, pair_start_index, pair_end_index)

                    no_explode = False
                    break
        if not no_explode:
            continue

        for i in range(len(expr)):
            c = expr[i]
            if c in ["[", "]", ","]:
                continue

            num_start_index = i
            num_end_index = num_start_index + 1
            while expr[num_end_index] not in ["[", "]", ","]:
                num_end_index += 1
            num = int(expr[num_start_index:num_end_index])
            if num <= 9:
                continue

            first = num // 2
            second = num - first
            expr = expr[:num_start_index] + f"[{first},{second}]" + expr[num_end_index:]
            no_split = False
            break

        if no_split and no_explode:
            break
    return expr


def explode(expr, pair_start_index, pair_end_index):
    # explode
    first, second = [int(n) for n in expr[pair_start_index + 1:pair_end_index].split(",")]
    index_num_to_the_left_end = pair_start_index - 1
    while index_num_to_the_left_end > 0 and expr[index_num_to_the_left_end] in ["[", "]", ","]:
        index_num_to_the_left_end -= 1
    if index_num_to_the_left_end > 0:
        index_num_to_the_left_start = index_num_to_the_left_end
        while expr[index_num_to_the_left_start - 1] not in ["[", "]", ","]:
            index_num_to_the_left_start -= 1
        num_to_the_left = int(expr[index_num_to_the_left_start:index_num_to_the_left_end + 1])
        new_to_the_left = num_to_the_left + first
    index_num_to_the_right_start = pair_end_index + 1
    while index_num_to_the_right_start < len(expr) and expr[index_num_to_the_right_start] in ["[", "]", ","]:
        index_num_to_the_right_start += 1
    if index_num_to_the_right_start < len(expr):
        index_num_to_the_right_end = index_num_to_the_right_start
        while expr[index_num_to_the_right_end + 1] not in ["[", "]", ","]:
            index_num_to_the_right_end += 1
        num_to_the_right = int(expr[index_num_to_the_right_start:index_num_to_the_right_end + 1])
        new_to_the_right = num_to_the_right + second
    new_expr = ""
    if index_num_to_the_left_end > 0:
        new_expr += expr[:index_num_to_the_left_start] + str(new_to_the_left) \
                    + expr[index_num_to_the_left_end + 1:pair_start_index]
    else:
        new_expr += expr[:pair_start_index]
    new_expr += "0"
    if index_num_to_the_right_start < len(expr):
        new_expr += expr[pair_end_index + 1:index_num_to_the_right_start] + str(new_to_the_right) \
                    + expr[index_num_to_the_right_end + 1:]
    else:
        new_expr += expr[pair_end_index + 1:]
    expr = new_expr
    return expr


def reduce_pair(s1: str, s2: str) -> str:
    expr = f"[{s1},{s2}]"
    return reduce_one(expr)


def reduce(lines: list[str]) -> str:
    res = lines[0].strip()
    for line in lines[1:]:
        res = reduce_pair(res, line.strip())
    return res


def get_magnitude(red_res: str) -> int:
    if len(red_res) == 1:
        return int(red_res)
    assert red_res[-1] == "]"
    comma_index = 1
    nested_brackets = 0
    while comma_index < len(red_res) - 1:
        if nested_brackets == 0 and red_res[comma_index] == ",":
            break
        if red_res[comma_index] == "[":
            nested_brackets += 1
        if red_res[comma_index] == "]":
            nested_brackets -= 1
        comma_index += 1
    if comma_index == len(red_res):
        raise Exception("comma not found")
    left = get_magnitude(red_res[1:comma_index])
    right = get_magnitude(red_res[comma_index + 1:-1])
    my_val = 3 * left + 2 * right
    return my_val


def run(lines: list[str]) -> int:
    max_mag = 0
    for l1 in lines:
        for l2 in lines:
            if l1 == l2:
                continue
            red = reduce_pair(l1.strip(), l2.strip())
            magnitude = get_magnitude(red)
            if magnitude > max_mag:
                max_mag = magnitude
    return max_mag


def test() -> None:
    txt = """[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"""
    magnitude = run(txt.splitlines())
    assert magnitude == 3993


if __name__ == "__main__":
    test()
    main()
