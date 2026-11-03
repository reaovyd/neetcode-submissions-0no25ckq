class Solution:
    def islandsAndTreasure(self, grid: List[List[int]]) -> None:
        n = len(grid)
        m = len(grid[0])

        q = deque()
        visited = [[False for _ in range(m)] for _ in range(n)]

        for i in range(n):
            for j in range(m):
                if grid[i][j] == 0:
                    q.append([i, j, 0])
                    visited[i][j] = True
        dists = [(0, -1), (0, 1), (-1, 0), (1, 0)]
        while q:
            n_len = len(q)
            for _ in range(n_len):
                coords = q.popleft()
                for dist in dists:
                    dx = coords[0] + dist[0]
                    dy = coords[1] + dist[1]
                    if dx < 0 or dy < 0 or dx >= n or dy >= m or grid[dx][dy] == -1:
                        continue
                    if visited[dx][dy]:
                        continue
                    grid[dx][dy] = coords[2] + 1
                    q.append([dx, dy, grid[dx][dy]])
                    visited[dx][dy] = True
                    
                    

        