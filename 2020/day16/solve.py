def parse_field(line):
    name,values = line.split(": ")
    v1,v2 = values.split(" or ")
    v11,v12 = tuple(map(int, v1.split("-")))
    v21,v22 = tuple(map(int, v2.split("-")))
    return {name: [(v11, v12), (v21, v22)]}

ranges = {}

def matching_fields(val):
    matches = []
    for key in ranges.keys():
        for (lo,hi) in ranges[key]:
            if lo <= val <= hi:
                matches.append(key)
                break
    return set(matches)

with open("day16.txt", "r") as fd:
    inp = fd.read().strip().split("\n\n")

    for field in inp[0].split("\n"):
        ranges |= parse_field(field)

    cnt = 0
    tickets = []
    for ticket in inp[2].split("\n")[1:]:
        nums = list(map(int, ticket.split(",")))
        tck_ok = True
        for n in nums:
            if len(matching_fields(n)) == 0:
                cnt += n
                tck_ok = False

        if tck_ok:
            tickets.append(nums)

    print("PART1 : ", cnt)

    possibilities = [set(ranges.keys()) for _ in range(len(ranges.keys()))]

    for idx,tck in enumerate(tickets):
        for field_idx,val in enumerate(tck):
            possibilities[field_idx] &= matching_fields(val)

    done = [0 for _ in range(len(ranges.keys()))]
    while any(len(s) > 0 for s in possibilities):
        for i,s in enumerate(possibilities):
            if len(s) == 1:
                done[i] = s.pop()
            else:
                for x in s.copy():
                    if x in done:
                        s.discard(x)

    my_nums = list(map(int, inp[1].split("\n")[1].split(",")))
    res = 1
    for i,f in enumerate(done):
        if f.startswith("departure"):
            res *= my_nums[i]
    print("PART2 : ", res)




