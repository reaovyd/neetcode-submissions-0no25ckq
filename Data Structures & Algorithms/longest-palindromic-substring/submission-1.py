class Solution:
    def longestPalindrome(self, s: str) -> str:
        n = len(s)
        palindrome_tup = [-1, -1]
        max_len = -1
        for i in range(n):
            j, k = i, i
            while j >= 0 and k < n and s[j] == s[k]:
                if k - j + 1 > max_len:
                    max_len = k - j + 1
                    palindrome_tup = [j, k + 1]
                j -= 1
                k += 1
            j, k = i, i + 1
            while j >= 0 and k < n and s[j] == s[k]:
                if k - j + 1 > max_len:
                    max_len = k - j + 1
                    palindrome_tup = [j, k + 1]
                j -= 1
                k += 1
        return s[palindrome_tup[0] : palindrome_tup[1]]