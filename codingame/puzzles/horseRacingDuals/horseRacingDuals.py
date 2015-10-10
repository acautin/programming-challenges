import sys
import math

n = int(raw_input())
pi = []
for i in xrange(n):
    pi.append(int(raw_input()))

pi.sort()

diff = int(math.fabs(pi[1] - pi[0]))
result = diff
prev = pi[1]

for i in xrange(n):
    if i > 1:
        diff = int(math.fabs(pi[i] - prev))
        if diff < result:
            result = diff
        prev = pi[i]

print result
