from threading import currentThread


with open("puzzleInput.txt") as f:
    prevLine = 0;
    count = 0;
    for line in f:
        currentLine = int(line.split("\n")[0]);
        currentLine = int(currentLine);
        if int(line.split("\n")[0]) > prevLine:
            count +=1;
        prevLine = int(line.split("\n")[0]);
        
    print(count);