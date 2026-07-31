class Solution:
    def dailyTemperatures(self, temperatures: List[int]) -> List[int]:
        # [(73, 0), (74, 1)]
        ans = [0 for _ in range(len(temperatures))]
        st = []
        for (i, temp) in enumerate(temperatures):
            while len(st) > 0 and temp > st[-1][0]:
                val = st.pop()[1]
                ans[val] = i - val
            st.append((temp, i))
        return ans