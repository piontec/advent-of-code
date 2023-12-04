from dataclasses import dataclass
from typing import List


@dataclass
class Data3d:
    x: int = 0
    y: int = 0
    z: int = 0


@dataclass
class SpaceObject:
    position: Data3d
    velocity: Data3d

    def get_total_energy(self) -> int:
        potential = 0
        kinetic = 0
        potential += abs(self.position.x)
        potential += abs(self.position.y)
        potential += abs(self.position.z)
        kinetic += abs(self.velocity.x)
        kinetic += abs(self.velocity.y)
        kinetic += abs(self.velocity.z)
        return potential * kinetic


class Sim:
    def __init__(self, lines: List[str]):
        super().__init__()
        self.objects = self.parse(lines)

    def parse(self, lines: List[str]) -> List[SpaceObject]:
        res: List[SpaceObject] = []
        for line in lines:
            params = [p.strip() for p in line.strip('<>\n').split(',')]
            assert len(params) == 3
            x = int(params[0].split('=')[1])
            y = int(params[1].split('=')[1])
            z = int(params[2].split('=')[1])
            obj = SpaceObject(Data3d(x, y, z), Data3d(0, 0, 0))
            res.append(obj)
        return res

    def run(self, steps: int):
        for step in range(steps):
            self._apply_gravity()
            self._apply_velocity()

    def _apply_gravity(self):
        for i1 in range(len(self.objects)):
            o1 = self.objects[i1]
            for i2 in range(i1 + 1, len(self.objects)):
                o2 = self.objects[i2]
                delta1, delta2 = self._compute_delta(
                    o1.position.x, o2.position.x)
                o1.velocity.x += delta1
                o2.velocity.x += delta2
                delta1, delta2 = self._compute_delta(
                    o1.position.y, o2.position.y)
                o1.velocity.y += delta1
                o2.velocity.y += delta2
                delta1, delta2 = self._compute_delta(
                    o1.position.z, o2.position.z)
                o1.velocity.z += delta1
                o2.velocity.z += delta2

    def _compute_delta(self, pos1: int, pos2: int) -> (int, int):
        if pos1 == pos2:
            return (0, 0)
        if pos1 < pos2:
            return (1, -1)
        return (-1, 1)

    def _apply_velocity(self):
        for o in self.objects:
            o.position.x += o.velocity.x
            o.position.y += o.velocity.y
            o.position.z += o.velocity.z

    def get_total_energy(self) -> int:
        total = 0
        for o in self.objects:
            total += o.get_total_energy()
        return total


lines = ["<x=-1, y=0, z=2>",
         "<x=2, y=-10, z=-7>",
         "<x=4, y=-8, z=8>",
         "<x=3, y=5, z=-1>"]
sim = Sim(lines)
sim.run(10)
energy = sim.get_total_energy()
assert energy == 179
with open("i12.txt") as f:
    lines = f.readlines()
sim = Sim(lines)
sim.run(1000)
energy = sim.get_total_energy()
print("Part 1: {}".format(energy))
