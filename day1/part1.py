with open("input", "r") as f:
    most = max([sum([int(x or 0) for x in l.split("\n")]) for l in f.read().split("\n\n")])
    print(most)
