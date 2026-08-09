class LRUCacheList:
    def __init__(self) -> None:
        self.head = None
        self.tail = None

    def push_node_tail(self, node: LRUCacheNode) -> None:
        if self.head is None:
            self.head = node
            self.tail = node
        else:
            node.prev = self.tail
            self.tail.next = node
            self.tail = node

    def pop_node_head(self) -> Optional[LRUCacheNode]:
        if self.head is None:
            return None
        else:
            node_head = self.head
            next_node = self.head.next
            self.head.next = None
            if next_node is not None:
                next_node.prev = None
            else:
                self.tail = None
            self.head = next_node
            return node_head

    def remove_node(self, node: LRUCacheNode):
        if node.prev and node.next:
            node.prev.next = node.next
            node.next.prev = node.prev
        elif node.prev and node.next is None:
            node.prev.next = None
            self.tail = node.prev
        elif node.prev is None and node.next:
            self.head = node.next
            self.head.prev = None
        else:
            self.head = None
            self.tail = None
        node.prev = None
        node.next = None

class LRUCacheNode:
    def __init__(
        self,
        val: int = 0,
        next: Optional[LRUCacheNode] = None,
        prev: Optional[LRUCacheNode] = None,
    ):
        self.val = val
        self.next = next
        self.prev = prev


class LRUCache:
    def __init__(self, capacity: int):
        self.cache = {}
        self.cache_list = LRUCacheList()
        self.capacity = capacity

    def get(self, key: int) -> int:
        if key not in self.cache:
            return -1
        else:
            value = self.cache[key][1]
            node = self.cache[key][0]
            self.cache_list.remove_node(node)
            self.cache_list.push_node_tail(node)
            return value

    def put(self, key: int, value: int) -> None:
        if key in self.cache:
            node = self.cache[key][0]
            self.cache[key] = (node, value)
            self.cache_list.remove_node(node)
            self.cache_list.push_node_tail(node)
        else:
            if len(self.cache) >= self.capacity:
                node = self.cache_list.pop_node_head()
                del self.cache[node.val]
            node = LRUCacheNode(key)
            self.cache[key] = (node, value)
            self.cache_list.push_node_tail(node)
