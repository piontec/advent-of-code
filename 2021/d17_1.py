from dataclasses import dataclass
from typing import Tuple


@dataclass
class Pair:
    x: int
    y: int


@dataclass
class Area:
    left_up: Pair
    right_down: Pair


def main() -> None:
    res = run(Area(left_up=Pair(217, -69), right_down=Pair(240, -126)))
    print(res)


def get_range(speed: Pair, area: Area) -> Tuple[bool, int]:
    pos = Pair(0, 0)
    spd = speed
    max_y = 0
    hit = False
    while pos.y >= area.right_down.y and pos.x <= area.right_down.x and not hit:
        pos.y += spd.y
        pos.x += spd.x
        hit = area.right_down.y <= pos.y <= area.left_up.y and area.left_up.x <= pos.x <= area.right_down.x
        if hit:
            break
        if pos.y > max_y:
            max_y = pos.y
        if spd.x > 0:
            spd.x -= 1
        spd.y -= 1
    return hit, max_y


def run(area: Area) -> int:
    max_y = 0
    for y_speed in range(1, abs(area.right_down.y + 2)):
        for x_speed in range(1, area.right_down.x + 2):
            hit, y = get_range(Pair(y_speed, x_speed), area)
            if hit and y > max_y:
                max_y = y
    return max_y


def test() -> None:
    res = run(Area(left_up=Pair(20, -5), right_down=Pair(30, -10)))
    assert res == 45


if __name__ == "__main__":
    test()
    main()
