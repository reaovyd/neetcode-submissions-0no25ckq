class Solution:
    def isNStraightHand(self, hand: list[int], groupSize: int) -> bool:
        n = len(hand)
        if n % groupSize != 0:
            return False
        hand = sorted(hand)
        ctr = defaultdict(int)
        for h in hand:
            ctr[h] += 1

        for h in hand:
            if ctr[h] == 0:
                continue
            else:
                for i in range(h, h + groupSize):
                    if i not in ctr or ctr[i] == 0:
                        return False
                    ctr[i] -= 1
        return True
