subj_num = 7


def find_loop_size(key: int) -> int:
    val = 1
    loop_size = 0
    while True:
        loop_size += 1
        val = (subj_num * val) % 20201227
        if val == key:
            break
    return loop_size


def iterate_loops(key: int, loops: int) -> int:
    val = 1
    for i in range(loops):
        val = (key * val) % 20201227
    return val


def get_encryption_key(card_pub_key: str, door_pub_key: str) -> int:
    card_pub = int(card_pub_key)
    door_pub = int(door_pub_key)
    door_loop_size = find_loop_size(door_pub)
    card_loop_size = find_loop_size(card_pub)
    shared_key = iterate_loops(door_pub, card_loop_size)
    return shared_key


card_pub_key = "5764801"
door_pub_key = "17807724"
res1 = get_encryption_key(card_pub_key, door_pub_key)
assert res1 == 14897079

with open("i25.txt", "r") as f:
    lines = f.readlines()
res = get_encryption_key(lines[0].strip(), lines[1].strip())
print(res)
