use std::collections::HashSet;

// Enum to represent colors
#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Blue,
    Green,
}

// Struct to represent a Game
#[derive(Debug)]
pub struct Game {
    pub id: usize,
    cubes: Vec<(Color, usize)>,
}

impl Game {
    fn new() -> Self {
        Self {
            id: 0,
            cubes: vec![]
        }
    }

    // Constructor for Game
    pub fn parse(input: &str) -> Self {
        let mut game = Self::new();

        let mut ignore_chars: HashSet<char> = HashSet::new();
        ignore_chars.insert(':');
        ignore_chars.insert(',');
        ignore_chars.insert(';');

        let input: String = input.chars().filter(|&char| !ignore_chars.contains(&char)).collect();
        let words: Vec<&str> = input.split_whitespace().collect();

        for chunk in words.chunks(2) {
            let (word1, word2) = (chunk[0], chunk[1]);

            match (word1, word2) {
                ("Game", id_str) => game.id = id_str.parse().unwrap_or_else(|e| panic!("Failed to parse ID: {}", e)),
                (_, color_str) => {
                    let color = match color_str {
                        "red" => Color::Red,
                        "blue" => Color::Blue,
                        "green" => Color::Green,
                        _ => panic!("Invalid color"),
                    };
                    game.cubes.push((color, word1.parse().unwrap_or_else(|e| panic!("Failed to parse cube number: {}", e))));
                }
            }
        }

        game
    }


    pub fn is_valid(&self, reds: usize, greens: usize, blues: usize) -> bool {
        self.cubes.iter().all(|(color, number)| {
            match color {
                Color::Red => number <= &reds,
                Color::Blue => number <= &blues,
                Color::Green => number <= &greens,
            }
        })
    }

    pub fn find_power(&self) -> usize {
        let mut reds: usize = 0;
        let mut greens: usize = 0;
        let mut blues: usize = 0;

        self.cubes.iter().for_each(|(color, number)| {
            match color {
                Color::Red => { if number > &reds { reds = *number } },
                Color::Blue => { if number > &blues { blues = *number } },
                Color::Green => { if number > &greens { greens = *number } },
            }
        });

        reds * greens * blues
    }
}