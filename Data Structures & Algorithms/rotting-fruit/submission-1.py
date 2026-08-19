class Solution:
    def orangesRotting(self, grid: List[List[int]]) -> int:
        n, m = len(grid), len(grid[0])
        q = deque()
        for i in range(n):
            for j in range(m):
                if grid[i][j] == 2:
                    q.append((i, j))
        dists = [(-1, 0), (1, 0), (0, -1), (0, 1)]
        ans = 0
        while q:
            ln = len(q)
            found = False
            for _ in range(ln):
                coord = q.popleft()
                for dist in dists:
                    dx = coord[0] + dist[0]
                    dy = coord[1] + dist[1]
                    if dx < 0 or dy < 0 or dx >= n or dy >= m or grid[dx][dy] == 2 or grid[dx][dy] == 0:
                        continue
                    grid[dx][dy] = 2
                    q.append((dx, dy))
                    found = True
            if found:
                ans += 1

        fresh = 0
        for i in range(n):
            for j in range(m):
                if grid[i][j] == 1:
                    fresh += 1
        if fresh > 0:
            return -1
        return ans