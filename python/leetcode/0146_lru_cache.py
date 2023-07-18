from collections import OrderedDict

class LRUCache:
    def __init__(self, capacity: int):
        # key -> value
        self.__cache = OrderedDict()
        self.__capacity = capacity
                
    def get(self, key: int) -> int:
        if key in self.__cache:
            self.__cache.move_to_end(key)
            return self.__cache[key]
        else:
            return -1

    def put(self, key: int, value: int) -> None:
        self.__cache[key] = value
        self.__cache.move_to_end(key)
        
        if len(self.__cache) > self.__capacity:
            self.__cache.popitem(False)            
        

