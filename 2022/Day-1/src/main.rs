use std::vec;

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let str = include_str!("input.txt");
    let mut sums: Vec<i32> = vec![];

    let mut num: i32 = 0;

    for line in str.lines() {
        if line == "" {
            sums.push(num);
            num = 0;
        } else {
            num += line.parse::<i32>().unwrap_or(0)
        }
    }
    sums.push(num);

    sums.sort(); sums = sums.into_iter().rev().collect();
    println!("Part One: {}", sums[0]);
}

fn part_two() {
    let str = include_str!("input.txt");
    let mut sums: Vec<i32> = vec![];

    let mut num: i32 = 0;

    for line in str.lines() {
        if line == "" {
            sums.push(num);
            num = 0;
        } else {
            num += line.parse::<i32>().unwrap_or(0)
        }
    }
    sums.push(num);

    sums.sort(); sums = sums.into_iter().rev().collect();

    num = 0;

    for i in &sums[0..3] {
        num += i;
    }

    println!("Part Two: {}", num);
}