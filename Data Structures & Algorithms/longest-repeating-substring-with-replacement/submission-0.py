class Solution:
    def characterReplacement(self, s: str, k: int) -> int:
        def get_ord_val(c: str) -> int:
            return ord(c) - ord("A")

        alph = [0 for _ in range(26)]
        i, j = 0, 0
        n = len(s)
        ans = 0
        while j < n:
            alph[get_ord_val(s[j])] += 1
            mx = max(alph)
            win = j - i + 1
            if win - mx > k:
                alph[get_ord_val(s[i])] -= 1
                i += 1
            else:
                ans = max(ans, win)
            j += 1
        return ans
