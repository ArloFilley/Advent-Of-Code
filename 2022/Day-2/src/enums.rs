#[derive(PartialEq)]
pub enum Move {
    Rock,
    Paper,
    Scissors
}

impl Move {
    pub fn comp(two: Move, one: Move) -> i32 {
        let mut num: i32 = 0;

        if one == two {

        }

        match one {
            Move::Rock => { num += 1 }
            Move::Paper => { num += 2 }
            Move::Scissors => { num += 3 }
        }

        if  one == Move::Rock && two == Move::Scissors
            || one == Move::Scissors && two == Move::Paper
            || one == Move::Paper && two == Move::Rock {
            
            num += 6;
        } else if one == two {
            num += 3;
        } else {
            num += 0;
        } 

        // println!("{num}");
        num
    }
}