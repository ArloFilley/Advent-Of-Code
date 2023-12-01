fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let str = include_str!("input.txt");
    let mut score = 0;

    for line in str.lines() {
        let tasks: Vec<&str> = line.split(',').collect();
        let one_lower: i32 = tasks[0].split('-').collect::<Vec<&str>>()[0].parse().unwrap();
        let one_upper: i32 = tasks[0].split('-').collect::<Vec<&str>>()[1].parse().unwrap();

        let two_lower: i32 = tasks[1].split('-').collect::<Vec<&str>>()[0].parse().unwrap();
        let two_upper: i32 = tasks[1].split('-').collect::<Vec<&str>>()[1].parse().unwrap();

        if one_lower <= two_lower && one_upper >= two_upper {
            score += 1;
        } else if two_lower <= one_lower && two_upper >= one_upper {
            score += 1;
        }

    }

    println!("Part One: {score}");
}

fn part_two() {
    let str = include_str!("input.txt");
    let mut score = 0;

    for line in str.lines() {
        let tasks: Vec<&str> = line.split(',').collect();
        let one_range: std::ops::Range<i32> = tasks[0].split('-').collect::<Vec<&str>>()[0].parse::<i32>().unwrap()..(tasks[0].split('-').collect::<Vec<&str>>()[1].parse::<i32>().unwrap()+1);
        let two_range: std::ops::Range<i32> = tasks[1].split('-').collect::<Vec<&str>>()[0].parse::<i32>().unwrap()..(tasks[1].split('-').collect::<Vec<&str>>()[1].parse::<i32>().unwrap()+1);

        'this: for i in one_range {
            if two_range.contains(&i) {
                score += 1;
                break 'this;
            }
        }
    }

    println!("Part Two: {score}");
}