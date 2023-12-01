#[derive(Debug)]
pub struct Tree {
    pub height: i32,
    pub visible: bool,
}

use std::collections::HashMap;

impl Tree {
    pub fn check_left(trees: &HashMap<(i32, i32), Tree>, location: (i32, i32)) -> bool {
        for x in (0..location.0).rev() {
            // println!("{} {} {}", trees.get(&(x, location.1)).unwrap().height, 
            //     trees.get(&location).unwrap().height, 
            //     trees.get(&(x, location.1)).unwrap().height >= trees.get(&location).unwrap().height
            // );
            if trees.get(&(x, location.1)).unwrap().height >= trees.get(&location).unwrap().height {
                return false;
            }
        }
        
        true
    }

    pub fn check_right(trees: &HashMap<(i32, i32), Tree>, size: i32, location: (i32, i32)) -> bool {
        for x in location.0+1..size + 1 {
            // println!("Location: {:?} Checking: {:?} \nTree 1: {} Tree 2: {} Tree >= tree 2: {}\n", 
            //     location,
            //     (x, location.1),
                
            //     trees.get(&(x, location.1)).unwrap().height, 
            //     trees.get(&location).unwrap().height, 
            //     trees.get(&(x, location.1)).unwrap().height >= trees.get(&location).unwrap().height
            // );
            if trees.get(&(x, location.1)).unwrap().height >= trees.get(&location).unwrap().height {
                return false;
            }
        }
        
        true
    }

    pub fn check_up(trees: &HashMap<(i32, i32), Tree>, location: (i32, i32)) -> bool {
        for y in (0..location.1).rev() {
            // println!("Location: {:?} Checking: {:?} \nTree 1: {} Tree 2: {}\nTree 1 >= Tree 2: {}", 
            //     location,
            //     (location.0, y),
                
            //     trees.get(&(location.0, y)).unwrap().height, 
            //     trees.get(&location).unwrap().height, 
            //     trees.get(&(location.0, y)).unwrap().height >= trees.get(&location).unwrap().height,
            // );
            if trees.get(&(location.0, y)).unwrap().height >= trees.get(&location).unwrap().height {
                return false;
            }
        }
        
        true
    }

    pub fn check_down(trees: &HashMap<(i32, i32), Tree>, size: i32, location: (i32, i32)) -> bool {
        for y in (location.1 + 1)..size+1 {
            // println!("Location: {:?} Checking: {:?} \nTree 1: {} Tree 2: {}\nTree 1 >= Tree 2: {}\n", 
            //     location,
            //     (location.0, y),
                
            //     trees.get(&(location.0, y)).unwrap().height, 
            //     trees.get(&location).unwrap().height, 
            //     trees.get(&(location.0, y)).unwrap().height >= trees.get(&location).unwrap().height,
            // );
            if trees.get(&(location.0, y)).unwrap().height >= trees.get(&location).unwrap().height {
                return false;
            }
        }
        
        true
    }
    
}

impl Tree {
    pub fn check_num_left(trees: &HashMap<(i32, i32), Tree>, location: (i32, i32)) -> i32 {
        let mut num: i32 = 0;
        for x in (0..location.0).rev() {
            if trees.get(&(x, location.1)).unwrap().height >= trees.get(&location).unwrap().height {
                return num + 1;
            } else {
                num += 1;
            }
        }
        
        num
    }

    pub fn check_num_right(trees: &HashMap<(i32, i32), Tree>, size: i32, location: (i32, i32)) -> i32 {
        let mut num: i32 = 0;
        for x in location.0+1..size + 1 {
            if trees.get(&(x, location.1)).unwrap().height >= trees.get(&location).unwrap().height {
                return num + 1;
            } else {
                num += 1;
            }   
        }
        
        num
    }

    pub fn check_num_up(trees: &HashMap<(i32, i32), Tree>, location: (i32, i32)) -> i32 {
        let mut num: i32 = 0;
        for y in (0..location.1).rev() {
            if trees.get(&(location.0, y)).unwrap().height >= trees.get(&location).unwrap().height {
                return num + 1;
            } else {
                num += 1;
            }
        }
        
        num
    }

    pub fn check_num_down(trees: &HashMap<(i32, i32), Tree>, size: i32, location: (i32, i32)) -> i32 {
        let mut num: i32 = 0;
        for y in (location.1 + 1)..size+1 {
            if trees.get(&(location.0, y)).unwrap().height >= trees.get(&location).unwrap().height {
                return num + 1;
            } else {
                num += 1;
            }
        }
        
        num
    }
}