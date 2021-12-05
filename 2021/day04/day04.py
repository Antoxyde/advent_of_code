import re

def win(board, nums):

    arr = [[0 for _ in range(5)] for _ in range(5)]
    win = False

    for n in nums:
        for r in range(5):
            for c in range(5):
                if board[r][c] == n:
                    arr[r][c] = 1
    
    # Lines
    for r in range(5):
        if set(arr[r]) == set([1]):
            win = True

    # Cols
    for c in range(5):
        if set([arr[r][c] for r in range(5)]) == set([1]):
            win = True 

    if win:
        b = sum(board,[])
        return (sum(b) - sum(set(b).intersection(nums)))  * nums[-1]
    else:
        return 0


def part1(boards, numbers):

    done = [] 

    for i in numbers:
        done.append(i)
        for board in boards:
            ret = win(board, done)
            if ret > 0:
                print("PART 1: ", ret)
                return


def part2(boards, numbers):
    bn = list(range(len(boards)))
    
    done = []
    for i in numbers:
        done.append(i)
        for b in bn:
            ret = win(boards[b], done)
            if ret > 0:
                bn.remove(b)
            
            if len(bn) == 0:
                print("PART 2:", ret)
                return

with open("day04.txt") as fd:
    lines = [l for l in fd.read().split("\n") if l != '']

    numbers = list(map(int, lines[0].split(",")))

    print(numbers)
    boards = []
    for i in range(1, len(lines), 5):
        board = list(map(lambda x:  list(map(int, re.findall("\d+", x))), lines[i:i+5]))
        boards.append(board)
    part1(boards, numbers)
    part2(boards, numbers)
