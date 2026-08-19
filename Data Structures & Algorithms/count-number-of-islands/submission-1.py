class Solution:
    def numIslands(self, grid: List[List[str]]) -> int:
        row_len, col_len = len(grid), len(grid[0])
        ans = 0
        def dfs(row: int, col: int):
            if row < 0 or col < 0 or row >= row_len or col >= col_len or grid[row][col] == '0':
                return
            grid[row][col] = '0'
            dfs(row - 1, col)
            dfs(row + 1, col)
            dfs(row, col - 1)
            dfs(row, col + 1)
            
        for row in range(row_len):
            for col in range(col_len):
                if grid[row][col] == '1':
                    dfs(row, col)
                    ans += 1
        return ans
        