class Solution:

    def encode(self, strs: List[str]) -> str:
        ans = []
        for s in strs:
            s_ln = str(len(s))
            ans.append(s_ln + "_" + s)
        return ''.join(ans)


    def decode(self, s: str) -> List[str]:
        i = 0
        n = len(s)
        ans = []
        while i < n:
            j = i
            while j < n and s[j] != '_':
                j += 1
            ln = int(s[i:j])
            j += 1
            if ln == 0:
                ans.append("")
                i = j
                continue
            new_s = []
            while j < n and ln > 0:
                new_s.append(s[j])
                j += 1
                ln -= 1
            ans.append(''.join(new_s))
            i = j
        return ans


            



