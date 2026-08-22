class Solution:
    def uniquePaths(self, m: int, n: int) -> int:
        dp = [[0 for j in range(n + 1)] for i in range(m + 1)]
        dp[m][n] = 1

        for i in range(m, 0, -1):
            for j in range(n, 0, -1):
                if i + 1 < m + 1:
                    dp[i][j] += dp[i+1][j]
                if j + 1 < n + 1:
                    dp[i][j] += dp[i][j + 1]

        return dp[1][1]