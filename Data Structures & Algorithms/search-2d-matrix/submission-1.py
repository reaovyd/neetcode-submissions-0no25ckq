class Solution:
    def searchMatrix(self, matrix: List[List[int]], target: int) -> bool:
        n = len(matrix)

        def findRow(matrix: List[List[int]], target: int) -> int:
            # 34
            # 1 10 23
            # 0 3
            # m = 3 / 2 = 1
            # L = m = 1
            # R = 3
            # 34 > 23
            # 5 / 2 =2
            ret = n - 1
            i, j = 0, n - 1
            while i <= j:
                m = (i + j) // 2
                if matrix[m][0] <= target:
                    ret = m
                    i = m + 1
                else:
                    j = m - 1
            return ret

        def binarySearch(row: List[int], target: int) -> bool:
            m = len(row)
            i, j = 0, m - 1
            while i <= j:
                m = (i + j) // 2
                if row[m] < target:
                    i = m + 1
                elif row[m] > target:
                    j = m - 1
                else:
                    return True
            return False

        row = findRow(matrix, target)
        return binarySearch(matrix[row], target)
