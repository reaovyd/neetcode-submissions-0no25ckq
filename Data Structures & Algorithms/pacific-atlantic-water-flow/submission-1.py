class Solution:
    def pacificAtlantic(self, heights: List[List[int]]) -> List[List[int]]:
        n, m = len(heights), len(heights[0])
        atlantic_queue = deque()
        pacific_queue = deque()

        atlantic = [[False for _ in range(m)] for _ in range(n)]
        pacific = [[False for _ in range(m)] for _ in range(n)]

        for j in range(m):
            if not atlantic[n - 1][j]:
                atlantic_queue.append((n - 1, j))
            if not pacific[0][j]:
                pacific_queue.append((0, j))
            atlantic[n - 1][j] = True
            pacific[0][j] = True

        for i in range(n):
            if not atlantic[i][m - 1]:
                atlantic_queue.append((i, m - 1))
            if not pacific[i][0]:
                pacific_queue.append((i, 0))
            atlantic[i][m - 1] = True
            pacific[i][0] = True
        
        dists = [(0, -1), (0, 1), (1, 0), (-1, 0)]

        while atlantic_queue:
            nn = len(atlantic_queue)
            for i in range(nn):
                coords = atlantic_queue.popleft()
                for dist in dists:
                    dx = dist[0] + coords[0]
                    dy = dist[1] + coords[1]
                    if dx < 0 or dy < 0 or dx >= n or dy >= m or atlantic[dx][dy] or heights[coords[0]][coords[1]] > heights[dx][dy]:
                        continue
                    atlantic[dx][dy] = True
                    atlantic_queue.append((dx, dy))


        while pacific_queue:
            nn = len(pacific_queue)
            for i in range(nn):
                coords = pacific_queue.popleft()
                for dist in dists:
                    dx = dist[0] + coords[0]
                    dy = dist[1] + coords[1]
                    if dx < 0 or dy < 0 or dx >= n or dy >= m or pacific[dx][dy] or heights[coords[0]][coords[1]] > heights[dx][dy]:
                        continue
                    pacific[dx][dy] = True
                    pacific_queue.append((dx, dy))

        ans = []
        for i in range(n):
            for j in range(m):
                if atlantic[i][j] and pacific[i][j] == atlantic[i][j]:
                    ans.append([i, j])
        return ans                    