class Solution:
    def checkValidString(self, s: str) -> bool:
        op = []
        stars = []

        for (i, c) in enumerate(s):
            if c == ')':
                if op:
                    op.pop()
                elif stars:
                    stars.pop()
                else:
                    return False
            elif c == '(':
                op.append(i)
            elif c == '*':
                stars.append(i)
        
        while op:
            if stars and op[-1] < stars[-1]:
                op.pop()
                stars.pop()
            else:
                return False

        return True

        