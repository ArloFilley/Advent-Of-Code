fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let input = include_str!("./proper/input.txt");

    for i in 0..input.len() - 3 {
        let one = input.chars().nth(i).unwrap();
        let two  = input.chars().nth(i + 1).unwrap();
        let three  = input.chars().nth(i + 2).unwrap();
        let four  = input.chars().nth(i + 3).unwrap();

        if !(one == two || one == three || one == four || two == three || two == four || three == four) {
            println!("Part One: {}", i + 4);
            break;
        }
    }
}

fn part_two() {
    let input = include_str!("./proper/input.txt");

    for i in 0..input.len() - 13 {
        let mut arr: Vec<char> = vec![];
        'inner: for j in 0..14 {
            let character = input.chars().nth(i + j).unwrap();
            if arr.contains(&character) {
                break 'inner;
            } else {
                arr.push(character)
            }
        }

        if arr.len() == 14 {
            println!("Part Two: {}", i + 14);
            break;
        }
    }
}