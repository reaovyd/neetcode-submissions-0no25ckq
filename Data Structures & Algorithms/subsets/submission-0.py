class Solution:
    def subsets(self, nums: List[int]) -> List[List[int]]:
        ans = []
        n = len(nums)
        def _subsets(i: int, curr: List[int]):
            if i == n:
                ans.append(list(curr))
            else:
                curr.append(nums[i])
                _subsets(i + 1, curr)
                curr.pop()
                _subsets(i + 1, curr)
        _subsets(0, [])
        return ans