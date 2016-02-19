#! /usr/bin/env python3

import threading


# The easiest way to pass an integer by reference in Python is to define a
# class to hold it. We could've just used a global, but using globals in Rust
# come with extra restrictions, and we want the two examples to look similar.
class Counter:
    def __init__(self):
        self.count = 0


def increment_counter(counter, times):
    for i in range(times):
        counter.count += 1


counter = Counter()

# Start a thread that increments the counter 500k times.
thread = threading.Thread(target=lambda: increment_counter(counter, 500000))
thread.start()

# At the same time, increment the counter 500k times in the main thread.
# RACE CONDITION!
increment_counter(counter, 500000)
thread.join()

print("We incremented this counter a million times. What's it's value now?")
print("=>", counter.count)
