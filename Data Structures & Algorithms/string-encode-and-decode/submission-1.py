class Solution:
    def encode(self, strs: List[str]) -> str:
        ans = []
        for s in strs:
            if len(s) == 0:
                ans.append("0")
                ans.append("_")
                ans.append("")
            else:
                ans.append(str(len(s)))
                ans.append("_")
                ans.append(s)
        return "".join(ans)

    def decode(self, s: str) -> List[str]:
        ans = []
        i = 0
        n = len(s)
        while i < n:
            ln = []
            while i < n and s[i] != "_":
                ln.append(s[i])
                i += 1
            actual_len = int("".join(ln))
            i += 1
            if actual_len == 0:
                ans.append("")
            else:
                actual_s = []
                while i < n and actual_len > 0:
                    actual_s.append(s[i])
                    actual_len -= 1
                    i += 1
                ans.append("".join(actual_s))
        return ans
