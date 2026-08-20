class Solution:
    def solve(self, board: List[List[str]]) -> None:
        n, m = len(board), len(board[0])
        should_not_fill = [[False for _ in range(m)] for _ in range(n)]
        coords_o_queue = deque()
        for i in range(n):
            if board[i][0] == 'O':
                coords_o_queue.append((i, 0))
                should_not_fill[i][0] = True
            if board[i][m - 1] == 'O':
                coords_o_queue.append((i, m - 1))
                should_not_fill[i][m - 1] = True
        for j in range(m):
            if board[0][j] == 'O':
                coords_o_queue.append((0, j))
                should_not_fill[0][j] = True
            if board[n - 1][j] == 'O':
                coords_o_queue.append((n - 1, j))
                should_not_fill[n - 1][j] = True
        
        dists = [(0, -1), (0, 1), (1, 0), (-1, 0)]
        
        while coords_o_queue:
            mm = len(coords_o_queue)
            for _ in range(mm):
                coords = coords_o_queue.popleft()
                for dist in dists:
                    dx = coords[0] + dist[0]
                    dy = coords[1] + dist[1]
                    if dx < 0 or dy < 0 or dx >= n or dy >= m or should_not_fill[dx][dy] or board[dx][dy] == 'X':
                        continue
                    should_not_fill[dx][dy] = True
                    coords_o_queue.append((dx, dy))
        
        for i in range(n):
            for j in range(m):
                if board[i][j] == 'O' and not should_not_fill[i][j]:
                    board[i][j] = 'X'
