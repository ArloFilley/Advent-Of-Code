use std::vec;

fn main() {
    part_two();
}

fn part_one() {
    let input = include_str!("input/input.txt");

    let mut instructions: Vec<Instruction> = vec![];
    for line in input.lines() {
        if line == "noop" {
            instructions.push(Instruction::Noop)
        } else {
            let val: i32 = line.split(" ").nth(1).unwrap().parse().unwrap();
            instructions.push(Instruction::Addx(val))
        }
    }

    let mut cpu = CPU { cycles: 0, x: 1, crt: vec![] };
    let mut results: Vec<i32> = vec![];
    let mut total_cycles = 0;

    for instruction in instructions {
        match cpu.instruct(&instruction) {
            None => {},
            Some(x) => { results.push(x) } 
        }

        match instruction {
            Instruction::Noop => { total_cycles += 1 }
            Instruction::Addx(_) => { total_cycles += 2 }
        }
    }       

    let mut result = 0;

    results.iter().for_each(|num| result += num );

    // println!("{results:#?}");
    // println!("{total_cycles}");
    println!("{result}")
}

fn part_two() {
    let input = include_str!("input/input.txt");

    let mut instructions: Vec<Instruction> = vec![];
    for line in input.lines() {
        if line == "noop" {
            instructions.push(Instruction::Noop)
        } else {
            let val: i32 = line.split(" ").nth(1).unwrap().parse().unwrap();
            instructions.push(Instruction::Addx(val))
        }
    }

    let mut cpu = CPU { cycles: 0, x: 1, crt: vec![] };
    let mut results: Vec<i32> = vec![];
    let mut total_cycles = 0;

    for instruction in instructions {
        cpu.instruct2(&instruction);

        match instruction {
            Instruction::Noop => { total_cycles += 1 }
            Instruction::Addx(_) => { total_cycles += 2 }
        }
    }       

    let mut result = 0;

    results.iter().for_each(|num| result += num );

    // println!("{results:#?}");
    // println!("{total_cycles}");
    // println!("{cpu:?}")

    println!();

    for y in (0..240).step_by(40) {
        for x in 0..40 {
            print!("{}", cpu.crt[x + y]);
        }
        print!("\n");
    }
}

#[derive(Debug)]
enum Instruction {
    Addx(i32),
    Noop
}

#[derive(Default, Debug)]
struct CPU {
    cycles: i32,
    x: i32,
    crt: Vec<char>
}

impl CPU {
    pub fn instruct(&mut self, instruction: &Instruction) -> Option<i32> {
        match instruction {
            Instruction::Noop => { 
                self.cycles += 1; 

                if (self.cycles - 20) % 40 == 0 {
                    return Some(self.x * self.cycles);
                }
                
                return None
            },
            Instruction::Addx(x) => { 
                self.cycles += 1; 
                if (self.cycles - 20) % 40 == 0 {
                    let return_val: i32 = self.x * self.cycles;

                    self.cycles += 1;

                    self.x += x;
                    return Some(return_val)
                } else {
                    self.cycles += 1;
                    if (self.cycles - 20) % 40 == 0 {
                        let return_val: i32 = self.x * self.cycles;
    
                        self.x += x;
                        return Some(return_val)
                    } else {
                        self.x += x;
                        return None
                    }
                }
            }
        }
    }

    pub fn instruct2(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Noop => { 
                self.draw_crt_pixel();
                self.cycles += 1; 
            },
            Instruction::Addx(x) => { 
                self.draw_crt_pixel();
                self.cycles += 1; 
                self.draw_crt_pixel();
                self.cycles += 1;
                self.x += x;
            }
        }
    }

    fn draw_crt_pixel(&mut self) {
        let current_pixel = (self.crt.len() as i32) % 40;
        
        // println!("{}..{} | {} |> {} <|> if", self.x-1, self.x+2, current_pixel, (self.x-1..self.x+2).contains( &current_pixel ));println!("{}..{} | {} |> {} <|> if", self.x-1, self.x+2, current_pixel, (self.x-1..self.x+2).contains( &current_pixel ));

        if (self.x-1..self.x+2).contains( &current_pixel ) {
            self.crt.push('#');
        } else {
            self.crt.push('.');
        }
    }
}