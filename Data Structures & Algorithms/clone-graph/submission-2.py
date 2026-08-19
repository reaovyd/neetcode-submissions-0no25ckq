"""
# Definition for a Node.
class Node:
    def __init__(self, val = 0, neighbors = None):
        self.val = val
        self.neighbors = neighbors if neighbors is not None else []
"""

from typing import Optional
class Solution:
    def cloneGraph(self, node: Optional['Node']) -> Optional['Node']:
        # mp = {node_val, (cloned_node, neighbors)}
        if node is None:
            return None
        node_map = {}
        q = []
        q.append(node)
        while len(q) > 0:
            node = q.pop()
            vals = []
            for neighbor in node.neighbors:
                if neighbor.val not in node_map:
                    q.append(neighbor)
                vals.append(neighbor.val)
            node_map[node.val] = [Node(node.val), vals]
        for node_val, tupl in node_map.items():
            neighbors = []
            for val in tupl[1]:
                neighbors.append(node_map[val][0])
            tupl[0].neighbors = neighbors
        return node_map[1][0]