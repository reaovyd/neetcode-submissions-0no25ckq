class Solution:
    def mergeTriplets(self, triplets: list[list[int]], target: list[int]) -> bool:
        ans = [-1, -1, -1]
        for triplet in triplets:
            if triplet[0] > target[0] or triplet[1] > target[1] or triplet[2] > target[2]:
                continue
            ans = [max(ans[0], triplet[0]), max(ans[1], triplet[1]), max(ans[2], triplet[2])]
        if -1 in ans:
            return False
        return ans == target