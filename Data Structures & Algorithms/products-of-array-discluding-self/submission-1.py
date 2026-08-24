class Solution:
    def productExceptSelf(self, nums: List[int]) -> List[int]:
        # 1, 1, 1 2, 1 2 3
        n = len(nums)
        output = []
        p = 1
        for i in range(n):
            output.append(p)
            p *= nums[i]
        p = 1
        for i in range(n - 1, -1, -1):
            output[i] *= p
            p *= nums[i]

        return output