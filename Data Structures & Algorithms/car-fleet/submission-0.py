class Solution:
    def carFleet(self, target: int, position: List[int], speed: List[int]) -> int:
        # position + time * speed >= target
        # time >= (target - position) / speed
        times = map(lambda x: (target - x[0]) / x[1], sorted(zip(position, speed)))
        st = []
        for time in times:
            while len(st) > 0 and time >= st[-1]:
                st.pop()
            st.append(time)
        return len(st)
