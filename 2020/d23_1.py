from typing import Tuple, Optional


class Node:
    def __init__(self, val: int):
        self.val: int = val
        self.next: Optional["Node"] = None
        # self.prev = None

    def __str__(self):
        return str(self.val)


class CrabRing:
    def __init__(self, str_nums: str):
        nums = [int(c) for c in str_nums]
        self.nums = nums
        self.min_val = min(nums)
        self.max_val = max(nums)
        prev = Node(nums[0])
        first = prev
        for num in nums[1:]:
            next = Node(num)
            prev.next = next
            # next.prev = prev
            prev = next
        next.next = first
        # first.prev = next
        self.current = first

    def cut_out(self, length: int) -> Tuple[Node, Node]:
        start = self.current.next
        end = start
        for _ in range(length - 1):
            end = end.next
        self.current.next = end.next
        return start, end

    def insert(self, node: Node, chain: Tuple[Node, Node]) -> None:
        old_next = node.next
        node.next = chain[0]
        chain[1].next = old_next

    def res_string(self) -> str:
        node_1 = self.current
        while node_1.val != 1:
            node_1 = node_1.next

        res = ""
        cur_node = node_1.next
        while cur_node != node_1:
            res += str(cur_node.val)
            cur_node = cur_node.next
        return res

    def crab_moves(self, moves: int) -> str:
        length = 3
        for i in range(moves):
            cut = self.cut_out(length)
            dest_val = self.current.val
            # search for destination node
            dest_node = None
            while True:
                dest_val -= 1
                if dest_val < self.min_val:
                    dest_val = self.max_val
                elif dest_val not in self.nums:
                    continue
                cut_cur, cut_last = cut
                num_in_cut = False
                while cut_cur != cut_last:
                    if cut_cur.val == dest_val:
                        num_in_cut = True
                        break
                    cut_cur = cut_cur.next
                if cut_cur.val == dest_val or num_in_cut:
                    continue
                dest_node = self.current
                while dest_node.val != dest_val:
                    dest_node = dest_node.next
                break
            self.insert(dest_node, cut)
            self.current = self.current.next
        return self.res_string()


test1 = "389125467"
cr1 = CrabRing(test1)
res1_1 = cr1.crab_moves(10)
assert res1_1 == "92658374"
cr1 = CrabRing(test1)
res1_2 = cr1.crab_moves(100)
assert res1_2 == "67384529"
cr = CrabRing("284573961")
print(cr.crab_moves(100))
