class Solution:
    def islandsAndTreasure(self, grid: List[List[int]]) -> None:
        n, m = len(grid), len(grid[0])
        visited = [[False for _ in range(m)] for _ in range(n)]
        q = deque()
        for i in range(n):
            for j in range(m):
                if grid[i][j] == 0:
                    q.append([i, j, 0])
                    visited[i][j] = True
        dists = [(-1, 0), (1, 0), (0, -1), (0, 1)]
        while q:
            ln = len(q)
            for _ in range(ln):
                coords = q.popleft()
                for dist in dists:
                    dx = dist[0] + coords[0]
                    dy = dist[1] + coords[1]
                    cur_dist = coords[2]
                    if dx < 0 or dy < 0 or dx >= n or dy >= m or visited[dx][dy] or grid[dx][dy] == -1:
                        continue
                    grid[dx][dy] = cur_dist + 1
                    q.append([dx, dy, cur_dist + 1])
                    visited[dx][dy] = True

        
            
