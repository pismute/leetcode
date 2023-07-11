import collections.abc
import time

def assert_equal(a, b, msg="assert_equal"):
    start_time = time.time()
    
    if type(a) == list:
        a.sort()
        b.sort()

    assert a == b, f"{a} == {b}"

    print(f"{msg}: --- {time.time() - start_time} seconds ---")
    

