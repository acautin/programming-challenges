import sys
import math

# Don't let the machines win. You are humanity's last hope...

def findRight(line, j):
    while j < len(line):
        if line[j] == '0':
            return j
        j = j + 1
    return -1

def findBottom(lines, i, j):
    while i < len(lines):
        if lines[i][j] == '0':
            return i
        i = i + 1
    return -1

width = int(input())  # the number of cells on the X axis
height = int(input())  # the number of cells on the Y axis
lines = []
for i in range(height):
    lines.append(raw_input())  # width characters, each either 0 or .


# Three coordinates: a node, its right neighbor, its bottom neighbor
for i in range(height):
    for j in range(width):
        if lines[i][j] == '0':
            ri = -1
            # Should scan from the next letter.
            rj = findRight(lines[i], j + 1)
            if rj != -1:
                ri = i
            di = findBottom(lines, i + 1, j)
            dj = -1
            if di != -1:
                dj = j
            # Print j -> x, i -> y
            print(str(j) + ' ' + str(i) + ' ' + str(rj) + ' ' + str(ri) + ' ' + str(dj) + ' ' + str(di))
        j = j + 1
    i = i + 1
