class Solution:
    def partitionLabels(self, s: str) -> list[int]:
        alph = [[-1, -1] for _ in range(26)]

        for (i, c) in enumerate(list(s)):
            idx = ord(c) - ord('a')
            if alph[idx][0] == -1:
                alph[idx][0] = i
            alph[idx][1] = i
        
        alph = list(filter(lambda tup: tup[0] != -1, alph))
        alph = sorted(alph, key=lambda elem: elem[0])
        
        def mergeIntervals(intervals: List[List[int]]):
            st = [intervals[0]]

            for i in range(1, len(intervals)):
                if intervals[i][0] <= st[-1][1]:
                    interval_out = st.pop()
                    interval_out[1] = max(interval_out[1], intervals[i][1])
                    st.append(interval_out)
                else:
                    st.append(intervals[i])
            return st
        ans = list(map(lambda tup: tup[1] - tup[0] + 1, mergeIntervals(alph)))
        

        return ans
            
