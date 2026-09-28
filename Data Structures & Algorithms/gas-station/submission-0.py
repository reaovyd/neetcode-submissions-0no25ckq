class Solution:
    def canCompleteCircuit(self, gas: list[int], cost: list[int]) -> int:
        lst = list(map(lambda tup: tup[0] - tup[1], zip(gas, cost)))
        n = len(lst)
        if sum(lst) < 0:
            return -1
        idx_ans = 0
        cur_sm = 0
        for i in range(n):
            cur_sm += lst[i]
            if cur_sm < 0:
                cur_sm = 0
                idx_ans = i + 1

        return idx_ans
