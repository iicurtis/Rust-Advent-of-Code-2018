import numpy as np
import matplotlib.pyplot as plt

x = []
v = []

with open('./input/2018/day10.txt') as f:
    lines = [l.rstrip('\n') for l in f]

for line in lines:
    x.append((int(line[10:16]), int(line[18:24])))
    v.append((int(line[36:38]), int(line[40:42])))

import numpy as np
x = np.array(x)
v = np.array(v)

import matplotlib.pyplot as plt

def extent(t):
    locs = x + t*v
    print(np.max(locs, axis=0) - np.min(locs, axis=0))
    return sum(np.max(locs, axis=0) - np.min(locs, axis=0))

extent(1)
