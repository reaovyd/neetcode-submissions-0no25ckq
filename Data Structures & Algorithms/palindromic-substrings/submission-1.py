class Solution:
    def countSubstrings(self, s: str) -> int:
        n = len(s)
        ans = 0
        for i in range(n):
            j, k = i, i
            while j >= 0 and k < n and s[j] == s[k]:
                ans += 1
                j -= 1
                k += 1
            j, k = i, i + 1
            while j >= 0 and k < n and s[j] == s[k]:
                ans += 1
                j -= 1
                k += 1
        return ans