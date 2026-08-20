class Solution:
    def carFleet(self, target: int, position: List[int], speed: List[int]) -> int:
        # position + speed * time = target
        # time = (target - position) / speed
        position = list(zip(position, speed))
        sorted_positions = sorted(position)
        
        times = [(target - p[0]) / p[1] for p in (sorted_positions)]
        st = deque()
        for time in times:
            while len(st) > 0 and time >= st[-1]:
                st.pop()
            st.append(time)
        return len(st)
        