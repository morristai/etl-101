class Node:
    def __init__(self, key=None, value=None):
        self.key = key
        self.value = value
        self.prev = None
        self.next = None

class LRUCache:
    def __init__(self, capacity):
        self.capacity = capacity
        self.cache = {}
        self.head = Node()
        self.tail = Node()
        self.head.next = self.tail
        self.tail.prev = self.head

    def _move_to_front(self, node):
        # Remove the node from its current position
        node.prev.next = node.next
        node.next.prev = node.prev
        
        # Move the node to the front
        node.next = self.head.next
        node.prev = self.head

        # Update the pointers
        self.head.next.prev = node
        self.head.next = node

    def get(self, key):
        if key in self.cache:
            node = self.cache[key]
            self._move_to_front(node)
            return node.value
        else:
            return None

    def put(self, key, value):
        if key in self.cache:
            node = self.cache[key]
            node.value = value
            self._move_to_front(node)
        else:
            if len(self.cache) == self.capacity:
                del self.cache[self.tail.prev.key]
                self.tail.prev = self.tail.prev.prev
                self.tail.prev.next = self.tail

            new_node = Node(key, value)
            self.cache[key] = new_node
            new_node.next = self.head.next
            new_node.prev = self.head
            self.head.next.prev = new_node
            self.head.next = new_node

    def display(self):
        current = self.head.next
        while current != self.tail:
            print(f"({current.key}, {current.value})", end=" ")
            current = current.next
        print()

# Example usage:
cache = LRUCache(2)
cache.put(1, 'A')
cache.put(2, 'B')
cache.display()  # Output: (2, B) (1, A)
cache.put(3, 'C')  # Evicts key 1
print(cache.get(1))  # Output: None
print(cache.get(2))  # Output: B
cache.display()  # Output: (2, B) (3, C)

