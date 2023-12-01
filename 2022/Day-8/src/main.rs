use std::collections::HashMap;
mod structs;
use crate::structs::Tree;

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let input = include_str!("./input/input.txt");

    let mut trees: HashMap<(i32, i32), Tree> = HashMap::new();

    let size: i32 = (input.lines().nth(0).unwrap().len() - 1) as i32;

    for (y, line) in input.lines().enumerate() {
        for (x, character) in line.chars().enumerate() {
            trees.insert(
                (x as i32, y as i32), 
                Tree { height: character.to_string().parse::<i32>().unwrap(), visible: false }
            );
            if x == 0 || x == size as usize  || y == 0 || y == size as usize {
                trees.get_mut(&(x as i32, y as i32)).unwrap().visible = true;
            }
        }
    }

    for y in 1..size {
        for x in 1..size {
            if
                Tree::check_left(&trees, (x, y)) || 
                Tree::check_right(&trees, size, (x, y)) || 
                Tree::check_up(&trees, (x,y)) || 
                Tree::check_down(&trees, size, (x, y))
            {
                trees.get_mut(&(x, y)).unwrap().visible = true;
            } else {
            }
        }
    }

    let mut result = 0;
    trees.values().for_each(|f| if f.visible { result += 1 } );

    println!("Part One: {result}")
}

fn part_two() {
    let input = include_str!("./input/input.txt");

    let mut trees: HashMap<(i32, i32), Tree> = HashMap::new();

    let size: i32 = (input.lines().nth(0).unwrap().len() - 1) as i32;

    for (y, line) in input.lines().enumerate() {
        for (x, character) in line.chars().enumerate() {
            trees.insert(
                (x as i32, y as i32), 
                Tree { height: character.to_string().parse::<i32>().unwrap(), visible: false }
            );
            if x == 0 || x == size as usize  || y == 0 || y == size as usize {
                trees.get_mut(&(x as i32, y as i32)).unwrap().visible = true;
            }
        }
    }

    let mut map: HashMap<(i32, i32), i32> = HashMap::new();

    for y in 0..size+1 {
        for x in 0..size+1 {
            let mut num = 1;
            num *= Tree::check_num_left(&trees, (x, y));
            num *= Tree::check_num_right(&trees, size, (x, y));
            num *= Tree::check_num_up(&trees, (x,y));
            num *= Tree::check_num_down(&trees, size, (x, y));

            map.insert((x, y), num);
        }
    }
    
    let mut map: Vec<i32> = map.into_values().collect(); map.sort(); map = map.into_iter().rev().collect();

    println!("Part Two: {}", map[0]);
}