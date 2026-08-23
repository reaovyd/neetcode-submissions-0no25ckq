class Solution:
    def lengthOfLIS(self, nums: List[int]) -> int:
        n = len(nums)
        dp = [0 for _ in range(n)]
        ans = 1
        dp[n - 1] = 1
        for i in range(n - 2, -1, -1):
            dp[i] = 1
            for j in range(i + 1, n, 1):
                if nums[i] < nums[j]:
                    dp[i] = max(dp[i], 1 + dp[j])
            ans = max(ans, dp[i])
        return ans
        