with open("puzzleInput.txt") as f:
    x = 0;
    y = 0;
    aim = 0;
    for line in f:
        split = line.split(" ");
        instruction = split[0];
        num = int(split[1]);
        if (instruction == "up"):
            aim -= num;
        elif (instruction == "down"):
            aim += num;
        else:
            y += num * aim;
            x += num;
            
    print(x,y,x*y);