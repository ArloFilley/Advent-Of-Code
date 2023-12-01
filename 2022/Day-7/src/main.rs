mod structs;
use crate::structs::Directory;

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let input = include_str!("./input/input.txt");

    let dir = Directory::create_directory_system(input);
    // println!("{dir:?}");
    let mut vec: Vec<i32> = vec![];
    dir.recursive_caller_1(&mut vec);

    let mut num = 0;
    for i in vec {
        num += i;
    }

    println!("Part One: {num}");

    // println!("{vec:?}");
}

fn part_two() {
    let input = include_str!("./input/input.txt");

    let dir = Directory::create_directory_system(input);
    // println!("space: {}, used space: {}, unused space: {}, needed space {}", 70_000_000, dir.size, 70_000_000 - dir.size, 30_000_000 - (70_000_000 - dir.size));


    let mut vec: Vec<i32> = vec![];
    dir.recursive_caller_2(&mut vec);

    vec.sort();

    println!("Part Two: {}", vec[0]);
}



