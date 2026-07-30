class Solution:
    def minWindow(self, s: str, t: str) -> str:
        m = len(t)
        n = len(s)
        if m > n:
            return ""
        ctr_t = defaultdict(int)

        for c in t:
            ctr_t[c] += 1

        i, j = 0, 0
        ctr_s = defaultdict(int)
        ans = (0, n + m)
        found = False
        matched = 0
        while j < n + 1:
            if j < n:
                ctr_s[s[j]] += 1
                if s[j] in ctr_t and ctr_s[s[j]] == ctr_t[s[j]]:
                    matched += 1
            j += 1
            while i < j and matched == len(ctr_t):
                if j - i <= ans[1] - ans[0]:
                    found = True
                    ans = (i, j)
                ctr_s[s[i]] -= 1
                if s[i] in ctr_t and ctr_s[s[i]] < ctr_t[s[i]]:
                    matched -= 1
                i += 1

        return s[ans[0] : ans[1]] if found else ""

        