mod enums;
use crate::enums::Move;

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let str = include_str!("input.txt");

    let mut total_score = 0;

    for line in str.lines() {

        let mut choices: (Move, Move) = (Move::Rock, Move::Rock);
        
        for char in line.split(" ") {
            // println!("{}", char.chars().nth(0).unwrap());
            match char.chars().nth(0).unwrap() {
                'A' => { choices.0 = Move::Rock }
                'B' => { choices.0 = Move::Paper }
                'C' => { choices.0 = Move::Scissors }
                'Y' => { choices.1 = Move::Paper }
                'X' => { choices.1 = Move::Rock }
                'Z' => { choices.1 = Move::Scissors }
                _ => {}
            }
        } 

        total_score += Move::comp(choices.0, choices.1);

    }

    // println!("{}", Move::comp(Move::Rock, Move::Paper));
    // println!("{}", Move::comp(Move::Paper, Move::Rock));
    // println!("{}", Move::comp(Move::Scissors, Move::Scissors));

    println!("{total_score}");
}

fn part_two() {
    let str = include_str!("input.txt");

    let mut total_score = 0;

    for line in str.lines() {

        let mut choices: (Move, Move) = (Move::Rock, Move::Rock);
        
        match line.chars().nth(0).unwrap() {
            'A' => { choices.0 = Move::Rock }
            'B' => { choices.0 = Move::Paper }
            'C' => { choices.0 = Move::Scissors }
                
            _ => {}
        }

        match (&choices.0, line.chars().nth(2).unwrap()) {
            (Move::Rock, 'X') => { choices.1 = Move::Scissors }
            (Move::Rock, 'Y') => { choices.1 = Move::Rock }
            (Move::Rock, 'Z') => { choices.1 = Move::Paper }

            (Move::Paper, 'X') => { choices.1 = Move::Rock }
            (Move::Paper, 'Y') => { choices.1 = Move::Paper }
            (Move::Paper, 'Z') => { choices.1 = Move::Scissors }

            (Move::Scissors, 'X') => { choices.1 = Move::Paper }
            (Move::Scissors, 'Y') => { choices.1 = Move::Scissors }
            (Move::Scissors, 'Z') => { choices.1 = Move::Rock }
            _ => {}
        }

        total_score += Move::comp(choices.0, choices.1);

    }

    println!("{total_score}");
}