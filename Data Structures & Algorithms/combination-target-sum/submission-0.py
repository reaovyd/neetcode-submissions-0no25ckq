class Solution:
    def combinationSum(self, candidates: List[int], target: int) -> List[List[int]]:
        ans = []
        n = len(candidates)
        def _combinationSum(n: int, curr_lst: List[int], curr_target: int):
            if n < 0 or curr_target < 0:
                return
            if curr_target == 0:
                ans.append(list(curr_lst))
            else:
                for i in range(n, -1, -1):
                    curr_lst.append(candidates[i])
                    _combinationSum(i, curr_lst, curr_target - candidates[i])
                    curr_lst.pop()
        _combinationSum(n - 1, [], target)
        return ans