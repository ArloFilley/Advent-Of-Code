from re import U

with open("puzzleInput.txt") as f:
    first = 0;
    second = 0;
    third = 0;
    prev = 0;
    count = 0;
    for line in f:
        third = second;
        second = first;
        first = int(line.split("\n")[0]);
        if (first + second + third > prev):
            count += 1;
        prev = (first + second + third);
        
    print(count-3);
        
        