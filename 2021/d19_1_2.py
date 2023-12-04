from typing import NamedTuple, Optional, Callable

from multiset import Multiset

Pos = NamedTuple('Pos', [('x', int), ('y', int), ('z', int)])


def distance(beacon1: Pos, beacon2: Pos) -> int:
    return abs(beacon1.x - beacon2.x) + abs(beacon1.y - beacon2.y) + abs(beacon1.z - beacon2.z)
    # return math.sqrt((beacon1.x - beacon2.x) ** 2 + (beacon1.y - beacon2.y) ** 2 + (beacon1.z - beacon2.z) ** 2)


class Scanner:
    def __init__(self, sid: int):
        self.sid = sid
        self.beacons: list[Pos] = []
        self.pos: Optional[Pos] = None
        self.beacon_dists: dict[int, Multiset] = {}

    def __str__(self) -> str:
        return str(self.sid)

    def __repr__(self) -> str:
        return self.__str__()

    def distance_to_scanner(self, o: 'Scanner') -> int:
        return abs(self.pos.x - o.pos.x) + abs(self.pos.y - o.pos.y) + abs(self.pos.z - o.pos.z)

    def compute_distances(self):
        for src_bid in range(len(self.beacons)):
            for dst_bid in range(len(self.beacons)):
                if src_bid == dst_bid:
                    continue
                if src_bid not in self.beacon_dists:
                    self.beacon_dists[src_bid] = Multiset()
                d = distance(self.beacons[src_bid], self.beacons[dst_bid])
                self.beacon_dists[src_bid].add(d)

    def find_candidates(self, unmatched: set['Scanner'], min_match: int = 12) -> set['Scanner']:
        res: set['Scanner'] = set()
        for unmatched_scanner in unmatched:
            matched = 0
            for src_bid in self.beacon_dists.keys():
                for dst_bid in unmatched_scanner.beacon_dists.keys():
                    common_dists = self.beacon_dists[src_bid].intersection(unmatched_scanner.beacon_dists[dst_bid])
                    if len(common_dists) >= min_match - 1:
                        matched += 1
                        if matched >= min_match:
                            res.add(unmatched_scanner)
                        break
                if matched >= min_match:
                    break
        return res

    def transform_beacons(self, transform_fun: Callable[[Pos], Pos]) -> 'Scanner':
        res = Scanner(self.sid)
        for b in self.beacons:
            res.beacons.append(transform_fun(b))
        res.beacon_dists = self.beacon_dists
        return res

    def get_rotations(self) -> list['Scanner']:
        rots = [
            lambda b: Pos(b.x, b.y, b.z),
            lambda b: Pos(b.x, -b.z, b.y),
            lambda b: Pos(b.x, -b.y, -b.z),
            lambda b: Pos(b.x, b.z, -b.y),

            lambda b: Pos(-b.y, b.x, b.z),
            lambda b: Pos(b.z, b.x, b.y),
            lambda b: Pos(b.y, b.x, -b.z),
            lambda b: Pos(-b.z, b.x, -b.y),

            lambda b: Pos(-b.x, -b.y, b.z),
            lambda b: Pos(-b.x, -b.z, -b.y),
            lambda b: Pos(-b.x, b.y, -b.z),
            lambda b: Pos(-b.x, b.z, b.y),

            lambda b: Pos(b.y, -b.x, b.z),
            lambda b: Pos(b.z, -b.x, -b.y),
            lambda b: Pos(-b.y, -b.x, -b.z),
            lambda b: Pos(-b.z, -b.x, b.y),

            lambda b: Pos(-b.z, b.y, b.x),
            lambda b: Pos(b.y, b.z, b.x),
            lambda b: Pos(b.z, -b.y, b.x),
            lambda b: Pos(-b.y, -b.z, b.x),

            lambda b: Pos(-b.z, -b.y, -b.x),
            lambda b: Pos(-b.y, b.z, -b.x),
            lambda b: Pos(b.z, b.y, -b.x),
            lambda b: Pos(b.y, -b.z, -b.x),
        ]
        res = [self.transform_beacons(r) for r in rots]
        return res

    def move(self, dx: int, dy: int, dz: int) -> 'Scanner':
        return self.transform_beacons(lambda b: Pos(b.x + dx, b.y + dy, b.z + dz))

    def matches(self, other: 'Scanner', min_match: int = 12) -> 'Scanner':
        for o in other.get_rotations():
            for src_bid in self.beacon_dists.keys():
                for dst_bid in o.beacon_dists.keys():
                    common_dists = self.beacon_dists[src_bid].intersection(o.beacon_dists[dst_bid])
                    if len(common_dists) >= min_match - 1:
                        dx = self.beacons[src_bid].x - o.beacons[dst_bid].x
                        dy = self.beacons[src_bid].y - o.beacons[dst_bid].y
                        dz = self.beacons[src_bid].z - o.beacons[dst_bid].z
                        moved = o.move(dx, dy, dz)
                        matching_beacons = set(self.beacons).intersection(set(moved.beacons))
                        if len(matching_beacons) >= min_match:
                            moved.pos = Pos(dx, dy, dz)
                            return moved


def run(lines: list[str]) -> tuple[int, int]:
    sid = 0
    scanner: Optional[Scanner] = None
    scanners: list[Scanner] = []
    for line in lines:
        if line.startswith("---"):
            scanner = Scanner(sid)
            continue
        if not line or line == '\n':
            scanners.append(scanner)
            sid += 1
            continue
        x, y, z = [int(p) for p in line.split(",")]
        pos = Pos(x, y, z)
        scanner.beacons.append(pos)
    scanners.append(scanner)
    scanners[0].pos = Pos(0, 0, 0)
    matched = {scanners[0]}
    unmatched = set(scanners[1:])
    for scanner in scanners:
        scanner.compute_distances()
    while len(unmatched) > 0:
        found_any = False
        for ms in matched:
            candidates = ms.find_candidates(unmatched)
            for c in candidates:
                c_matched = ms.matches(c)
                if c_matched:
                    matched.add(c_matched)
                    unmatched.remove(c)
                    found_any = True
                    break
            if found_any:
                break
        if not found_any:
            raise Exception('None of the candidates were matched')
    beacon_count: set[Pos] = set()
    for s in matched:
        beacon_count.update(s.beacons)
    max_dist = 0
    matched_scanners = list(matched)
    for i in range(len(matched_scanners)):
        for j in range(len(matched_scanners)):
            if i <= j:
                continue
            cur = matched_scanners[i].distance_to_scanner(matched_scanners[j])
            if cur > max_dist:
                max_dist = cur

    return len(beacon_count), max_dist


def main() -> None:
    with open("i19.txt", "r") as i:
        lines = i.readlines()
    res = run(lines)
    print(res)


def test2() -> None:
    lines = """--- scanner 0 ---
-618,-824,-621
-537,-823,-458
-447,-329,318
404,-588,-901
544,-627,-890
528,-643,409
-661,-816,-575
390,-675,-793
423,-701,434
-345,-311,381
459,-707,401
-485,-357,347

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
-476,619,847
-460,603,-452
729,430,532
-322,571,750
-355,545,-477
413,935,-424
-391,539,-444
553,889,-390
""".splitlines()
    res = run(lines)
    print(res)


def test() -> None:
    lines = """--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14""".splitlines()
    cnt, dist = run(lines)
    assert cnt == 79
    assert dist == 3621


if __name__ == "__main__":
    test()
    main()
