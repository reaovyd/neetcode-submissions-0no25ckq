class Solution:
    def combinationSum2(self, candidates: List[int], target: int) -> List[List[int]]:
        ans = []
        
        n = len(candidates)
        candidates = sorted(candidates)

        def _combinationSum2(i: int, lst: List[int], curr_target: int):
            if curr_target > target:
                return
            if i == n:
                if curr_target == target:
                    ans.append(list(lst))
            else:
                lst.append(candidates[i])
                _combinationSum2(i + 1, lst, curr_target + candidates[i])
                lst.pop()
                while i < n - 1:
                    if candidates[i] != candidates[i + 1]:
                        break
                    i += 1
                _combinationSum2(i + 1, lst, curr_target)
        _combinationSum2(0, [], 0)
        return ans
        