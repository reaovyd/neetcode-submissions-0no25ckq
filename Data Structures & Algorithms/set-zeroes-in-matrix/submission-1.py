class Solution:
    def setZeroes(self, matrix: list[list[int]]) -> None:
        n, m = len(matrix), len(matrix[0])
        q = deque()

        for i in range(n):
            for j in range(m):
                if matrix[i][j] == 0:
                    q.append((i, j))
        if len(q) == n * m:
            return
        while q:
            x, y = q.popleft()
            for i in range(0, x):
                matrix[i][y] = 0
            for i in range(x, n):
                matrix[i][y] = 0
            for j in range(0, y):
                matrix[x][j] = 0
            for j in range(y, m):
                matrix[x][j] = 0