class WordTrieNode:
    alphabet: List[Optional[WordTrieNode]]
    is_word: bool

    def __init__(self):
        self.alphabet = [None for _ in range(26)] 
        self.is_word = False

class WordDictionary:
    root: WordTrieNode

    def __init__(self):
        self.root = WordTrieNode()
        

    def addWord(self, word: str) -> None:
        root = self.root
        for c in word:
            idx = ord(c) - ord('a')
            if not root.alphabet[idx]:
                root.alphabet[idx] = WordTrieNode()
            root = root.alphabet[idx]
        root.is_word = True
        

    def search(self, word: str) -> bool:
        return self._search(self.root, word, 0)
    def _search(self, cur_root: WordTrieNode, word: str, idx: int) -> bool:
        if idx >= len(word):
            return False
        c = word[idx]
        ans = False
        if c == '.':
            for i in range(26):
                if cur_root.alphabet[i] is not None:
                    if cur_root.alphabet[i].is_word and idx == len(word) - 1:
                        return True
                    ans = ans or self._search(cur_root.alphabet[i], word, idx + 1)
            return ans
        else:
            if cur_root.alphabet[ord(c) - ord('a')] is not None:
                if cur_root.alphabet[ord(c) - ord('a')].is_word and idx == len(word) - 1:
                    return True
                return self._search(cur_root.alphabet[ord(c) - ord('a')], word, idx + 1)
            else:
                return False



        


# Your WordDictionary object will be instantiated and called as such:
# obj = WordDictionary()
# obj.addWord(word)
# param_2 = obj.search(word)