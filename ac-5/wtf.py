#!/usr/bin/env python3

data = [line for line in open("ac-5.txt").read().strip().split("\n")]
print(data)

seat_ids = []

for seat in data:
    row = int(seat[:7].replace("B", "1").replace("F", "0"), 2)
    col = int(seat[-3:].replace("R", "1").replace("L", "0"), 2)
    seat_ids.append(row * 8 + col)
print(f"Part 1: {max(seat_ids)}")
