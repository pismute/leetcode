import collections.abc
import time

def assert_equal(a, b, msg="assertEqual"):
    start_time = time.time()
    
    if type(a) == list:
        sort(a)
        sort(b)
        assert a == b
    else:
        assert a == b

    print(f"{msg}: --- {time.time() - start_time} seconds ---")
    

