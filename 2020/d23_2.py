from typing import Tuple, Optional, Dict


class Node:
    def __init__(self, val: int):
        self.val: int = val
        self.next: Optional["Node"] = None
        self.prev = None

    def __str__(self):
        return str(self.val)


class CrabRing:
    def __init__(self, str_nums: str):
        nums = [int(c) for c in str_nums]
        self.nums = nums
        self.min_val = min(nums)
        self.max_val = 1000000
        self.index: Dict[int, Node] = {}

        prev = Node(nums[0])
        self.index[nums[0]] = prev
        first = prev
        for num in nums[1:] + list(range(10, 1000000 + 1)):
            next = Node(num)
            self.index[num] = next
            prev.next = next
            next.prev = prev
            prev = next
        next.next = first
        first.prev = next
        self.current = first

    def cut_out(self, length: int) -> Tuple[Node, Node]:
        start = self.current.next
        end = start
        for _ in range(length - 1):
            end = end.next
        self.current.next = end.next
        end.next.prev = self.current
        return start, end

    def insert(self, node: Node, chain: Tuple[Node, Node]) -> None:
        old_next = node.next
        node.next = chain[0]
        chain[0].prev = node
        chain[1].next = old_next
        old_next.prev = chain[1]

    def res_string(self) -> str:
        node_1 = self.current
        while node_1.val != 1:
            node_1 = node_1.next
        res = node_1.next.val * node_1.next.next.val
        return str(res)

    def crab_moves(self, moves: int) -> str:
        length = 3
        for i in range(moves):
            if i % 10000 == 0:
                print(i)
            cut = self.cut_out(length)
            dest_val = self.current.val
            while True:
                dest_val = dest_val - 1
                if dest_val < self.min_val:
                    dest_val = self.max_val
                if dest_val in [cut[0].val, cut[0].next.val, cut[0].next.next.val]:
                    continue
                break
            dest_node = self.index[dest_val]
            self.insert(dest_node, cut)
            self.current = self.current.next
        return self.res_string()


test1 = "389125467"
cr1 = CrabRing(test1)
res1_2 = cr1.crab_moves(10 * 1000000)
assert res1_2 == "149245887792"
cr = CrabRing("284573961")
print(cr.crab_moves(10 * 1000000))
