class Solution:
    def maxSubArray(self, nums: List[int]) -> int:
        ans = float('-inf')
        cur = 0
        for num in nums:
            cur += num
            ans = max(ans, cur)
            if cur < 0:
                cur = 0
        return ans
        