class Solution:
    def checkInclusion(self, s1: str, s2: str) -> bool:
        def get_ord(s: str, idx: int) -> int:
            return ord(s[idx]) - ord("a")

        n, m = len(s1), len(s2)
        if n > m:
            return False
        alph1 = [0 for _ in range(26)]
        alph2 = [0 for _ in range(26)]

        for i in range(n):
            alph1[get_ord(s1, i)] += 1
            alph2[get_ord(s2, i)] += 1

        for i in range(n, m):
            if alph1 == alph2:
                return True
            alph2[get_ord(s2, i - n)] -= 1
            alph2[get_ord(s2, i)] += 1

        if alph1 == alph2:
            return True
        return False
