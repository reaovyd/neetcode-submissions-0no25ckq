class Solution:
    def canPartition(self, nums: List[int]) -> bool:
        sm = sum(nums)
        if sm % 2 != 0:
            return False
        half = sm // 2
        n = len(nums)
        sum_arr = [[False for _ in range(half + 1)] for _ in range(n + 1)]
        
        for i in range(n + 1):
            sum_arr[i][0] = True

        for i in range(1, n + 1):
            for j in range(1, half + 1):
                if j - nums[i - 1] >= 0:
                    sum_arr[i][j] = sum_arr[i - 1][j - nums[i - 1]] or sum_arr[i - 1][j]
                else:
                    sum_arr[i][j] = sum_arr[i - 1][j]
        
        
        ans = False
        for i in range(1, n + 1):
            ans = ans or sum_arr[i][half]
        return ans
