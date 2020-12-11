def print_seats(seats):
    print("----"*10)
    for l in seats:
        print("".join(l))
    print("----"*10)

def get_adjacents(seats, row, col, part1=False):
    cnt = 0

    for (a,b) in [(-1, -1), (-1, 0), (-1, +1), (0, -1), (0, +1), (+ 1, -1), (+1, 0), (+1,+1)]:
        r = row + a
        c = col + b
        while r >= 0 and r < len(seats) and c >= 0 and c < len(seats[0]):
            if seats[r][c] == '#':
               cnt += 1
               break
            elif seats[r][c] == 'L':
                break

            if part1:
                break
            r += a
            c += b

    return cnt



def tick(seats, part1=False):
    new = [l.copy() for l in seats]
    for r in range(len(seats)):
        for c in range(len(seats[0])):
            cnt = get_adjacents(seats, r, c, part1)
            if seats[r][c] == 'L' and cnt == 0:
                new[r][c] = '#'
            elif seats[r][c] == '#' and cnt >= (4 if part1 else 5):
                new[r][c] = 'L'

    return new

def part(old, part1=False):

    while True:
        new = tick(old, part1)

        if new == old:
            break

        old = new
    return "".join(map(lambda l : "".join(l), old)).count('#')



if __name__ == '__main__':
    seats = list(map(list, open("day11.txt").read().strip().split("\n")))
    print("PART1: {}".format(part(seats, True)))
    print("PART2: {}".format(part(seats, False)))
