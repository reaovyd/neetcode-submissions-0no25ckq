class MinStack:

    def __init__(self):
        self.st = []
        # [3, 2, 4, 5, 1]
        # [3, 2, 2, 2, 1]

    def push(self, value: int) -> None:
        if len(self.st) == 0:
            self.st.append((value, value))
        else:
            if self.st[-1][1] < value:
                self.st.append((value, self.st[-1][1]))
            else:
                self.st.append((value, value))

    def pop(self) -> None:
        self.st.pop()

    def top(self) -> int:
        return self.st[-1][0]

    def getMin(self) -> int:
        return self.st[-1][1]


# Your MinStack object will be instantiated and called as such:
# obj = MinStack()
# obj.push(value)
# obj.pop()
# param_3 = obj.top()
# param_4 = obj.getMin()