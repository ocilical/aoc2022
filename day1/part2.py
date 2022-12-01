with open("input", "r") as f:
    most = sum(list(sorted([sum([int(x or 0) for x in l.split("\n")]) for l in f.read().split("\n\n")], reverse=True))[:3])
    print(most)
