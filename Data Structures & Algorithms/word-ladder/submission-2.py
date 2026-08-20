class Solution:
    def ladderLength(self, beginWord: str, endWord: str, wordList: List[str]) -> int:
        word_list_set = set(wordList)
        if endWord not in word_list_set:
            return 0
        
        visited_words = set()
        q = deque()
        q.append((beginWord, 1))
        visited_words.add(beginWord)
        ans = 2 + len(wordList)
        found = False
        while q:
            mm = len(q)
            for _ in range(mm):
                word = q.popleft()
                if word[0] == endWord:
                    found = True
                    ans = min(ans, word[1])
                h_word = list(word[0])
                for l in range(len(word[0])):
                    for c in range(26):
                        char = chr(c + ord('a'))
                        if char == word[0][l]:
                            continue
                        h_word[l] = char
                        s_h_word = ''.join(h_word)
                        if s_h_word not in visited_words and s_h_word in word_list_set:
                            q.append((s_h_word, word[1] + 1))
                            visited_words.add(s_h_word)
                            
                        h_word[l] = word[0][l]
        if not found:
            return 0
        return ans 