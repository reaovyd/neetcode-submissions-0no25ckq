class Twitter:
    def __init__(self):
        self.tweets_per_user = {}
        self.follows_per_user = {}
        self.timestamp = 0

    def postTweet(self, userId: int, tweetId: int) -> None:
        if userId not in self.follows_per_user:
            self.follows_per_user[userId] = set()
        if userId not in self.tweets_per_user:
            self.tweets_per_user[userId] = deque()
        if len(self.tweets_per_user[userId]) >= 10:
            self.tweets_per_user[userId].popleft()
        self.tweets_per_user[userId].append((self.timestamp, tweetId))
        self.timestamp += 1


    def getNewsFeed(self, userId: int) -> list[int]:
        if userId not in self.follows_per_user:
            self.follows_per_user[userId] = set()
        prio_queue = []
        followers_and_self = self.follows_per_user[userId]
        followers_and_self.add(userId)
        for user in followers_and_self:
            if user not in self.tweets_per_user:
                continue
            for tweet in self.tweets_per_user[user]:
                if len(prio_queue) >= 10:
                    if tweet[0] > prio_queue[0][0]:
                        heapq.heappop(prio_queue)
                        heapq.heappush(prio_queue, tweet)
                else:
                    heapq.heappush(prio_queue, tweet)
        prio_queue = sorted(prio_queue, key=lambda pair: pair[0], reverse=True)
        return list(map(lambda pair: pair[1], prio_queue))

    def follow(self, followerId: int, followeeId: int) -> None:
        if followerId not in self.follows_per_user:
            self.follows_per_user[followerId] = set()
        self.follows_per_user[followerId].add(followeeId)

    def unfollow(self, followerId: int, followeeId: int) -> None:
        if followerId not in self.follows_per_user:
            self.follows_per_user[followerId] = set()
        self.follows_per_user[followerId].discard(followeeId)
        


# Your Twitter object will be instantiated and called as such:
# obj = Twitter()
# obj.postTweet(userId,tweetId)
# param_2 = obj.getNewsFeed(userId)
# obj.follow(followerId,followeeId)
# obj.unfollow(followerId,followeeId)