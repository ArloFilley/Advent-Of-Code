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

    games = games.drain(..).filter(|game| game.is_valid(12, 13, 14)).collect();
    println!("{}", games.iter().map(|game| game.id).sum::<usize>());
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

    games = games.drain(..).filter(|game| game.is_valid(12, 13, 14)).collect();
    println!("{}", games.iter().map(|game| game.id).sum::<usize>());
    //  println!("{:#?}", games);
}