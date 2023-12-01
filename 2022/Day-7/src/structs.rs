#[derive(Debug, Default, Clone)]
pub struct Directory {
    name: String,
    files: Vec<(i32, String)>,
    directories: Vec<Directory>,
    pub size: i32,
}

impl Directory {
    pub fn recursive_caller_1(&self, vec: &mut Vec<i32>) {
        if self.size <= 100_000 {
            // println!("{}", self.size);
            vec.push(self.size);
        }

        if self.directories.len() > 0 {
            for dir in self.directories.iter() {
                dir.recursive_caller_1(vec);
            }
        }
    }

    pub fn recursive_caller_2(&self, vec: &mut Vec<i32>) {
        if self.size >= 3837783 {
            // println!("{}", self.size);
            vec.push(self.size);
        }

        if self.directories.len() > 0 {
            for dir in self.directories.iter() {
                dir.recursive_caller_2(vec);
            }
        }
    }

    pub fn create_directory_system(input: &str) -> Self {
        let mut fs = Directory::default();
        fs.name = String::from("/");
        let root = fs;

        let mut dir_stack: Vec<Directory> = vec![root];

        for line in input.lines() {
            if line.contains("$ cd") && !(line == "$ cd ..") && !(line == "$ cd /") {
                let name = line.split("d ").collect::<Vec<&str>>()[1];

                dir_stack.push( Directory { name: name.into(), files: vec![], directories: vec![], size: 0 } );
            
            } else if line == "$ cd .." {
                let dir = dir_stack.pop().unwrap();
                dir_stack.last_mut().unwrap().size += dir.size;
                dir_stack.last_mut().unwrap().directories.push(dir);
            } else if line.chars().nth(0).unwrap().is_numeric() {
                let vals: Vec<&str> = line.split(' ').collect();
                dir_stack.last_mut().unwrap().files.push((vals[0].parse().unwrap(), vals[1].into()));
                dir_stack.last_mut().unwrap().size += vals[0].parse::<i32>().unwrap();
            }
        }

        let dir = dir_stack.pop().unwrap();
        dir_stack.last_mut().unwrap().size += dir.size;
        dir_stack.first_mut().unwrap().directories.push(dir);

        //println!("{dir_stack:?}");

        dir_stack.remove(0)
    }
}