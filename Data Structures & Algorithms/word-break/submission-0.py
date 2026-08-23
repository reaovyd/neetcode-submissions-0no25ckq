class Solution:
    def wordBreak(self, s: str, wordDict: List[str]) -> bool:
        n = len(s)
        word_dict_set = set(wordDict)
        cache = {}
        def _wordBreak(start: int) -> bool:
            if start >= n:
                return True
            if start in cache:
                return cache[start]
            ans = False
            for i in range(start, n):
                if s[start:i + 1] in word_dict_set:
                    ans = ans or _wordBreak(i + 1)
            cache[start] = ans
            return cache[start]
        return _wordBreak(0)
