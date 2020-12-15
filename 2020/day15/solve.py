
def run(bound, inp):

    nums = [0] + inp
    said = {}

    last = 0
    for i in range(1,bound+1):

        if i < len(nums):
            said[nums[i]] = (0,i)
            last = nums[i]
        else:
            old,curr = said[last]

            if old == 0:
                if 0 in said.keys():
                    _,curr = said[0]
                    said[0] = (curr,i)
                else:
                    said[0] = (0,i)
                last = 0
            else:
                diff = curr - old
                if not diff in said:
                    said[diff] = (0, i )
                else:
                    old,curr = said[diff]
                    said[diff] = (curr,i)
                last = diff
    return last
inp = [15,5,1,4,7,0]
print("PART1:", run(2020, inp))
print("PART2:", run(30000000, inp))
