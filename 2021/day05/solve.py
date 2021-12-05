import re

def part1(arr):
    for seg in inp.split("\n"):
        a,b,x,y = map(int, re.findall("\d+", seg))

        if not( a == x or b == y):
            continue

        while (a,b) != (x,y):
            arr[a][b] += 1
            if a < x: a+= 1
            if b < y : b += 1
            if a > x: a -= 1
            if b > y : b -= 1

        arr[a][b] += 1

    return len([i for i in sum(arr,[]) if i > 1])

def part2(arr):
    for seg in inp.split("\n"):
        a,b,x,y = map(int, re.findall("\d+", seg))
        arr[b][a] += 1
        while (a,b) != (x,y):
            if a < x: a+= 1
            if b < y : b += 1
            if a > x: a -= 1
            if b > y : b -= 1
            arr[b][a] += 1


    return len([i for i in sum(arr,[]) if i > 1])

inp = """0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"""

inp = open("day05.txt").read().strip()

print("PART 1:", part1([[0 for _ in range(1000)] for _ in range(1000)]))
print("PART 2:", part2([[0 for _ in range(1000)] for _ in range(1000)]))

