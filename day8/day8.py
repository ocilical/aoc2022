infile = open("input", "r")
trees = [[int(x) for x in list(l)] for l in infile.read().splitlines()]
infile.close()

visible = set()

maxH = [0] * len(trees[0])
for row in range(0, len(trees)):
    for col in range(0, len(trees[0])):
        if row == 0 or trees[row][col] > maxH[col]:
            maxH[col] = trees[row][col]
            visible.add((row, col))

maxH = [0] * len(trees[0])
for row in range(len(trees) - 1, -1, -1):
    for col in range(0, len(trees[0])):
        if row == (len(trees) - 1) or trees[row][col] > maxH[col]:
            maxH[col] = trees[row][col]
            visible.add((row, col))

maxH = [0] * len(trees)
for col in range(0, len(trees[0])):
    for row in range(0, len(trees)):
        if col == 0 or trees[row][col] > maxH[row]:
            maxH[row] = trees[row][col]
            visible.add((row, col))

maxH = [0] * len(trees)
for col in range(len(trees[0]) - 1, -1, -1):
    for row in range(0, len(trees)):
        if col == (len(trees[0]) - 1) or trees[row][col] > maxH[row]:
            maxH[row] = trees[row][col]
            visible.add((row, col))

print(f"part 1: {len(visible)}")

scores = [[0]*len(trees[0]) for _ in range(len(trees))]
for row in range(0, len(trees)):
    for col in range(0, len(trees[0])):
        totalScore = 1
        tempScore = 0
        for i in range(row + 1, len(trees)):
            tempScore += 1
            if trees[row][col] <= trees[i][col]:
                break
        totalScore *= tempScore
        tempScore = 0
        for i in range(row - 1, -1, -1):
            tempScore += 1
            if trees[row][col] <= trees[i][col]:
                break
        totalScore *= tempScore
        tempScore = 0
        for i in range(col + 1, len(trees[0])):
            tempScore += 1
            if trees[row][col] <= trees[row][i]:
                break
        totalScore *= tempScore
        tempScore = 0
        for i in range(col - 1, -1, -1):
            tempScore += 1
            if trees[row][col] <= trees[row][i]:
                break
        totalScore *= tempScore

        scores[row][col] = totalScore

print(max(map(max, scores)))
