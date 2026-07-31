class Solution:
    def evalRPN(self, tokens: List[str]) -> int:
        st = []
        for token in tokens:
            if token in "+-*/":
                op1 = int(st.pop())
                op2 = int(st.pop())
                if token == '+':
                    st.append(op2 + op1)
                elif token == '-':
                    st.append(op2 - op1)
                elif token == '*':
                    st.append(op2 * op1)
                else:
                    st.append(int(op2 / op1))
            else:
                st.append(token)
        return int(st[0])
