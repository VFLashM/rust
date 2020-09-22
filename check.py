#!/bin/env python3

import sys
import os
from collections import defaultdict, Counter

class Seq:
    def __init__(self):
        self.min = None
        self.max = 0
        self.cnt = 0
        self.sum = 0
        self.counter = Counter()

    def add(self, value):
        self.min = value if self.min is None else min(self.min, value)
        self.max = max(self.max, value)
        self.cnt += 1
        self.sum += value
        self.counter[value] += 1

locations = defaultdict(lambda: Seq())

root = sys.argv[1]
for name in os.listdir(root):
    path = os.path.join(root, name)
    with open(path) as f:
        for line in f:
            location, size = line.split()
            locations[location].add(int(size))

for loc in sorted(locations, key=lambda x: locations[x].cnt):
    seq = locations[loc]
    print('%s cnt=%s min=%s max=%s avg=%s mode=%s' % (loc, seq.cnt, seq.min, seq.max, float(seq.sum) / seq.cnt, seq.counter.most_common(1)[0]))
