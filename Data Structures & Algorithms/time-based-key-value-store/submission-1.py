class TimeMap:
    def __init__(self):
        self.map = {}
        pass

    def set(self, key: str, value: str, timestamp: int) -> None:
        if key not in self.map:
            self.map[key] = [(value, timestamp)]
        else:
            self.map[key].append((value, timestamp))

    def get(self, key: str, timestamp: int) -> str:
        if key not in self.map:
            return ""
        else:
            lst = self.map[key]
            n = len(lst)
            i, j = 0, n - 1
            ret = -1
            while i <= j:
                m = (i + j) // 2
                if lst[m][1] <= timestamp:
                    ret = m
                    i = m + 1
                else:
                    j = m - 1
            if ret == -1:
                return ""
            return lst[ret][0]
