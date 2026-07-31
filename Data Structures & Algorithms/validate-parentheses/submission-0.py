class Solution:
    def isValid(self, s: str) -> bool:
        st = []
        for c in s:
            if c in "({[":
                st.append(c)
            else:
                if len(st) == 0:
                    return False
                if st[-1] == "(" and c != ")":
                    return False
                elif st[-1] == "[" and c != "]":
                    return False
                elif st[-1] == "{" and c != "}":
                    return False
                st.pop()
        return len(st) == 0
