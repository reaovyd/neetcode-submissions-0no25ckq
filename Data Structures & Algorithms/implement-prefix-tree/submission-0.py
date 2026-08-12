class TrieNode:
    alph: List[Optional[TrieNode]]
    is_word: bool

    def __init__(self) -> None:
        self.alph = [None for _ in range(26)]
        self.is_word = False


class PrefixTree:
    root: TrieNode

    def __init__(self) -> None:
        self.root = TrieNode()

    def insert(self, word: str) -> None:
        root = self.root
        for c in word:
            cc = ord(c) - ord("a")
            if root.alph[cc] is not None:
                root = root.alph[cc]
            else:
                root.alph[cc] = TrieNode()
                root = root.alph[cc]
        root.is_word = True

    def search(self, word: str) -> bool:
        root = self.root
        for c in word:
            cc = ord(c) - ord("a")
            if root.alph[cc] is not None:
                root = root.alph[cc]
            else:
                return False
        return root.is_word

    def startsWith(self, prefix: str) -> bool:
        root = self.root
        for c in prefix:
            cc = ord(c) - ord("a")
            if root.alph[cc] is not None:
                root = root.alph[cc]
            else:
                return False
        return True
        
        