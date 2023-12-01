fn main() {
    // part_one();
    part_two();
}

fn priority(character: char) -> i32 {
    if character.is_uppercase() {
        return (character as u8 - 38) as i32;
    } else  {
        return (character as u8 - 96) as i32;
    }
}

fn part_one() {
    let str = include_str!("input.txt");
    let mut score = 0;
    
    for line in str.lines() {
        let first = &line[0..line.len() / 2];
        let second = &line[line.len() / 2..line.len()];

        'first: for char in first.chars() {
            if second.contains(char) {
                score += priority(char);
                break 'first;
            }
        }
    }

    println!("{score}");
}

fn part_two() {
    let str = include_str!("input.txt");
    let elves: Vec<&str> = str.split("\n").collect();
    let mut score = 0;

    for i in (0..elves.len()-1).step_by(3) {
        let one = elves[i];
        let two = elves[i+1];
        let three = elves[i+2];

        'second: for char in one.chars() {
            if two.contains(char) && three.contains(char) {
                score += priority(char);
                break 'second;
            }
        }
    }

    println!("{score}");
}
