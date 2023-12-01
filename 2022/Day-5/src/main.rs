fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let crates = include_str!("./proper/crates.txt");
    let crates: Vec<&str> = crates.lines().collect();
    let mut crates: Vec<String> = crates.into_iter().map(|s| s.into()).collect();

    let input = include_str!("./proper/input.txt");
    for line in input.lines() {
        let split: Vec<&str> = line.split(',').collect();
        let num_moved = split[0].parse::<usize>().unwrap();
        let from = split[1].parse::<usize>().unwrap() - 1;
        let into = split[2].parse::<usize>().unwrap() - 1;
        let mut temp = String::new();

        for _i in 0..num_moved {
            let char = crates[from].pop().unwrap();
            crates[into].push(char);
        }
    }

    let mut result = String::new();

    for each in crates {
        result.push(each.chars().nth(each.len() - 1).unwrap());
    }

    println!("Part One: {result}");
}

fn part_two() {
    let crates = include_str!("./proper/crates.txt");
    let crates: Vec<&str> = crates.lines().collect();
    let mut crates: Vec<String> = crates.into_iter().map(|s| s.into()).collect();

    let input = include_str!("./proper/input.txt");
    for line in input.lines() {
        let split: Vec<&str> = line.split(',').collect();
        let num_moved = split[0].parse::<usize>().unwrap();
        let from = split[1].parse::<usize>().unwrap() - 1;
        let into = split[2].parse::<usize>().unwrap() - 1;
        let mut temp = String::new();

        for _i in 0..num_moved {
            let char = crates[from].pop().unwrap();
            temp.push(char);
        }

        crates[into].push_str(&temp.chars().rev().collect::<String>())

    }

    let mut result = String::new();

    for each in crates {
        result.push(each.chars().nth(each.len() - 1).unwrap());
    }

    println!("Part Two: {result}");
}