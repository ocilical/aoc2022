inputfile = open("input", "r")
signal = inputfile.readline()
inputfile.close()

for i in range(0, len(signal) - 3):
    if len(set(signal[i:i+4])) == 4:
        print(f"part 1: {i}")
        break
