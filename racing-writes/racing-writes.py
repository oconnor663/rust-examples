#! /usr/bin/env python3

import threading

INCREMENTS_PER_THREAD = 100000
THREADS = 10


# Define a small class so that we can pass a counter by reference.
class Counter:
    def __init__(self):
        self.count = 0


# In Python, as in most languages, the easy way to increment a shared counter
# is prone to race conditions. Another thread can swoop in *after* we've read
# the previous value of count, but *before* we've written the new value. The
# result is that we overwrite the other thread's work, and the counter winds up
# smaller than expected.
def bad_increment(counter):
    counter.count += 1


counter = Counter()
threads = []
for _ in range(THREADS):
    def thread_inner():
        for _ in range(INCREMENTS_PER_THREAD):
            bad_increment(counter)
    thread = threading.Thread(target=thread_inner)
    threads.append(thread)
    thread.start()
for thread in threads:
    thread.join()
print("Expected:", THREADS * INCREMENTS_PER_THREAD)
print("Actual:", counter.count)
