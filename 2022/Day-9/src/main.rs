use std::{collections::HashSet, vec};

fn main() {
    part_two();
}


#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}


fn part_one() {
    let input = include_str!("./input/input.txt");

    let mut locations: HashSet<(i32, i32)> = HashSet::new();
    locations.insert((0,0));

    let mut moves: Vec<Direction> = vec![];
    for line in input.lines() {
        let splits: Vec<&str> = line.split(' ').collect();
        let direction: Option<Direction> = match splits[0] {
            "L" => { Some(Direction::Left) }
            "R" => { Some(Direction::Right) }
            "U" => { Some(Direction::Up) }
            "D" => { Some(Direction::Down) }
            _ => { None }
        };

        let direction = direction.unwrap();

        for _ in 0..splits[1].parse::<i32>().unwrap() {
            moves.push(direction.clone());
        }
    }

    let mut head: (i32, i32) = (0, 0);
    let mut tail: (i32, i32) = (0, 0);

    for direction in moves {
        match direction {
            Direction::Left => { head.0 -= 1 },
            Direction::Right => { head.0 += 1 },
            Direction::Up => { head.1 += 1 },
            Direction::Down => { head.1 -= 1 },
        }

        if (head.0 - tail.0).abs() > 1 || (head.1 - tail.1).abs() > 1 {
            match direction {
                Direction::Left => { tail.1 = head.1; tail.0 = head.0 + 1 },
                Direction::Right => { tail.1 = head.1; tail.0 = head.0 - 1 },
                Direction::Up => { tail.0 = head.0; tail.1 = head.1 - 1 },
                Direction::Down => { tail.0 = head.0; tail.1 = head.1 + 1},
            }
            locations.insert(tail);
        }
    }    

    /*
        ..##..
        ...##.
        .####.
        ....#.
        s###..
     */

    let mut num = 0;
    locations.iter().for_each(|_| num += 1);
    
    println!("Part One: {num}");
}

fn part_two() {
    let mut snake: Vec<(i32, i32)> = vec![(0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0)];
    let input = include_str!("./input/input.txt");

    let mut locations: HashSet<(i32, i32)> = HashSet::new();
    locations.insert((0,0));

    let mut moves: Vec<Direction> = vec![];
    for line in input.lines() {
        let splits: Vec<&str> = line.split(' ').collect();
        let direction: Option<Direction> = match splits[0] {
            "L" => { Some(Direction::Left) }
            "R" => { Some(Direction::Right) }
            "U" => { Some(Direction::Up) }
            "D" => { Some(Direction::Down) }
            _ => { None }
        };

        let direction = direction.unwrap();

        for _ in 0..splits[1].parse::<i32>().unwrap() {
            moves.push(direction.clone());
        }
    }

    for direction in moves {

        // Move the Head of the Rope
        match direction {
            Direction::Left => { snake[0].0 -= 1 },
            Direction::Right => { snake[0].0 += 1 },
            Direction::Up => { snake[0].1 += 1 },
            Direction::Down => { snake[0].1 -= 1 },
        }

        /*
            . . . . . .
            . . . . H .
            . . . . 1 .
            . 4 3 2 . .
            5 . . . . .  

            . . . . H .   |   . . . . H .
            . . . . 1 .   |   . . . . 1 .
            . . 4 3 2 .   |   . . . . 2 .
            . 5 . . . .   |   . 4 3 . . .
            6 . . . . .   |   5 . . . . .
        */

        // Move The rest of the rope

        for i in 0..snake.len() - 1 {

            let head = i;
            let tail = i + 1;

            let diff = (
                snake[head].0 - snake[tail].0,
                snake[head].1 - snake[tail].1
            );
            let not_touching = diff.0.abs() > 1 || diff.1.abs() > 1;
            
            if not_touching {
                snake[tail].0 += diff.0.signum();
                snake[tail].1 += diff.1.signum();

                if tail == 9 {
                    locations.insert(snake[tail]);
                }
            }
        }
    }

    let mut num = 0;
    locations.iter().for_each(|_| num += 1);
    
    println!("Part Two: {num}");
}