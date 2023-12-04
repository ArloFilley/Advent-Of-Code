use std::usize;
use crate::game::Game;
pub fn full() {
    // Read input from a file
    let input = include_str!("../input/input.txt");
    
    // println!("{}", input);

    // Create a Game instance from the input
    let mut games: Vec<Game> = vec![];
    for line in input.lines() {
        games.push(Game::parse(line));
    }

    println!("{}", games.iter().map(|game| game.find_power()).sum::<usize>());
}

pub fn example() {
    // Read input from a file
    let input = include_str!("../input/example_input.txt");
    
    // println!("{}", input);

    // Create a Game instance from the input
    let mut games: Vec<Game> = vec![];
    for line in input.lines() {
        games.push(Game::parse(line));
    }

    println!("{}", games.iter().map(|game| game.find_power()).sum::<usize>());
    //  println!("{:#?}", games);
}