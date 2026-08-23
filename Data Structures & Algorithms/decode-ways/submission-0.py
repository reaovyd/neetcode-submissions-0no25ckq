class Solution:
    def numDecodings(self, s: str) -> int:
        n = len(s)
        st = set()
        for i in range(26):
            st.add(str(i + 1))

        if n == 1:
            if s[0] in st:
                return 1
            else:
                return 0

        dp = [0] * n
        if s[0] in st:
            dp[0] = 1
        if s[1] in st:
            dp[1] = dp[0]
        if s[0:2] in st:
            dp[1] += 1

        
        for i in range(2, n):
            if s[i] in st:
                dp[i] += dp[i - 1]
            if s[i - 1:i+1] in st:
                dp[i] += dp[i - 2]
        return dp[n - 1]