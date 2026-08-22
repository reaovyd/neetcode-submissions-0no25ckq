class Solution:
    def rob(self, nums: List[int]) -> int:
        n = len(nums)
        if n == 1:
            return nums[0]
        dp = [float('-inf')] * n
        dp[n - 1] = nums[n - 1]
        dp[n - 2] = nums[n - 2]
        for i in range(n - 3, -1, -1):
            for j in range(i + 2, n):
                dp[i] = max(dp[i], nums[i] + dp[j])
        
        return max(dp[0], dp[1])
        