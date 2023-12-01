with open("puzzleInput.txt") as f:
    x = 0;
    y = 0;
    for line in f:
        split = line.split(" ");
        instruction = split[0];
        num = int(split[1]);
        if (instruction == "up"):
            y -= num;
        elif (instruction == "down"):
            y += num;
        else:
            x += num;
            
    print(x*y);