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


def get_range(speed: Pair, area: Area) -> bool:
    pos = Pair(0, 0)
    spd = speed
    hit = False
    while pos.y >= area.right_down.y and pos.x <= area.right_down.x and not hit:
        pos.y += spd.y
        pos.x += spd.x
        hit = area.right_down.y <= pos.y <= area.left_up.y and area.left_up.x <= pos.x <= area.right_down.x
        if hit:
            break
        if spd.x > 0:
            spd.x -= 1
        spd.y -= 1
    return hit


def run(area: Area) -> int:
    count = 0
    for y_speed in range(area.right_down.y, abs(area.right_down.y) + 1):
        for x_speed in range(1, area.right_down.x + 1):
            hit = get_range(Pair(x=x_speed, y=y_speed), area)
            if hit:
                count += 1
    return count


def test() -> None:
    res = run(Area(left_up=Pair(20, -5), right_down=Pair(30, -10)))
    assert res == 112


if __name__ == "__main__":
    test()
    main()
