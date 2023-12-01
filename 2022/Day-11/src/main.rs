fn main() {
    part_two();
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u128>,
    op: Operation,

    test: i32,
    throw_if_true: i32,
    throw_if_false: i32,
    inspections: i32
}

#[derive(Debug)]
enum Operation {
    Add(i32),
    Times(i32),
    Square,
}


fn lcm(first: i32, second: i32) -> i32 {
    first * second / gcd(first, second)
}

fn gcd(first: i32, second: i32) -> i32 {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}				

fn part_one() {
    let input: &str = include_str!("input/input.txt");
    let input: Vec<&str> = input.lines().collect();

    let mut monkeys: Vec<Monkey> = vec![];

    for i in (0..input.len()-2).step_by(7) {
        let starting_items_str: Vec<&str> = input[i + 1].split(": ").collect();
        let mut starting_items: Vec<u128> = vec![];
        for (i, item) in starting_items_str.into_iter().enumerate() {
            if i != 0 {
                item.split(", ").for_each( |item| starting_items.push(item.parse().unwrap()))
            }
        }
        // println!("{starting_items:?}");

        let operation_str: &str = input[i + 2].split("= ").collect::<Vec<&str>>()[1];
        let operation: Option<Operation>;
        // println!("{operation_str}");

        if operation_str == "old * old" {
            operation = Some(Operation::Square)
        } else if operation_str.contains('*') {
            let num: i32 = operation_str.split("* ").collect::<Vec<&str>>()[1].parse().unwrap();
            operation = Some(Operation::Times(num))
        } else if operation_str.contains('+') {
            let num: i32 = operation_str.split("+ ").collect::<Vec<&str>>()[1].parse().unwrap();
            operation = Some(Operation::Add(num))
        } else {
            operation = None;
        }

        let operation = operation.expect("Error finding operation");

        let test: i32 = input[i + 3].split("y ").collect::<Vec<&str>>()[1].parse().expect("Error Finding Test Devisor");
        let throw_if_true: i32 = input[i + 4].split("y ").collect::<Vec<&str>>()[1].parse().expect("Error Finding Test Devisor");
        let throw_if_false: i32 = input[i + 5].split("y ").collect::<Vec<&str>>()[1].parse().expect("Error Finding Test Devisor");

        monkeys.push(Monkey { items: starting_items, op: operation, test, throw_if_true, throw_if_false, inspections: 0 });
    }

    


    for _ in 0..20 {
        for monkey in 0..monkeys.len() {

            for item in (0..monkeys[monkey].items.len()).rev() {
                match monkeys[monkey].op {
                    Operation::Add(val) => { monkeys[monkey].items[item] += val as u128 },
                    Operation::Times(val) => { monkeys[monkey].items[item] *= val as u128 },
                    Operation::Square => { monkeys[monkey].items[item] *= monkeys[monkey].items[item] }
                }
                monkeys[monkey].inspections += 1;

                monkeys[monkey].items[item] /= 3;

                if monkeys[monkey].items[item] % monkeys[monkey].test as u128 == 0 {
                    let moving_item = monkeys[monkey].items.remove(item);
                    let monkey_true = monkeys[monkey].throw_if_true;

                    monkeys[monkey_true as usize].items.push(moving_item);
                } else {
                    let moving_item = monkeys[monkey].items.remove(item);
                    let monkey_false = monkeys[monkey].throw_if_false;

                    monkeys[monkey_false as usize].items.push(moving_item);
                }
            }
            
        }
    }

    let mut results = vec![];

    for monkey in monkeys {
        results.push(monkey.inspections);
    }

    results.sort();

    println!("{}", results[results.len() - 2] * results[results.len() - 1]);
}

fn part_two() {
    let input: &str = include_str!("input/input.txt");
    let input: Vec<&str> = input.lines().collect();

    let mut monkeys: Vec<Monkey> = vec![];

    for i in (0..input.len()-2).step_by(7) {
        let starting_items_str: Vec<&str> = input[i + 1].split(": ").collect();
        let mut starting_items: Vec<u128> = vec![];
        for (i, item) in starting_items_str.into_iter().enumerate() {
            if i != 0 {
                item.split(", ").for_each( |item| starting_items.push(item.parse().unwrap()))
            }
        }
        // println!("{starting_items:?}");

        let operation_str: &str = input[i + 2].split("= ").collect::<Vec<&str>>()[1];
        let operation: Option<Operation>;
        // println!("{operation_str}");

        if operation_str == "old * old" {
            operation = Some(Operation::Square)
        } else if operation_str.contains('*') {
            let num: i32 = operation_str.split("* ").collect::<Vec<&str>>()[1].parse().unwrap();
            operation = Some(Operation::Times(num))
        } else if operation_str.contains('+') {
            let num: i32 = operation_str.split("+ ").collect::<Vec<&str>>()[1].parse().unwrap();
            operation = Some(Operation::Add(num))
        } else {
            operation = None;
        }

        let operation = operation.expect("Error finding operation");

        let test: i32 = input[i + 3].split("y ").collect::<Vec<&str>>()[1].parse().expect("Error Finding Test Devisor");
        let throw_if_true: i32 = input[i + 4].split("y ").collect::<Vec<&str>>()[1].parse().expect("Error Finding Test Devisor");
        let throw_if_false: i32 = input[i + 5].split("y ").collect::<Vec<&str>>()[1].parse().expect("Error Finding Test Devisor");

        monkeys.push(Monkey { items: starting_items, op: operation, test, throw_if_true, throw_if_false, inspections: 0 });
    }

    let mut lcma = lcm(monkeys[0].test, monkeys[1].test) as u128;

    for monkey in 2..monkeys.len() {
        lcma = lcm(lcma as i32, monkeys[monkey].test) as u128;
    }

    println!("{}", lcma);


    for _ in 0..10_000 {
        for monkey in 0..monkeys.len() {
            // println!("{:?}", monkeys[monkey].items);

            for item in (0..monkeys[monkey].items.len()).rev() {
                match monkeys[monkey].op {
                    Operation::Add(val) => { 
                        monkeys[monkey].items[item] = monkeys[monkey].items[item] + val as u128; 
                    },
                    Operation::Times(val) => { 
                        monkeys[monkey].items[item] = monkeys[monkey].items[item] * val as u128; 
                    },
                    Operation::Square => { 
                        monkeys[monkey].items[item] *= monkeys[monkey].items[item] ; 
                    }
                }
                monkeys[monkey].inspections += 1;

                monkeys[monkey].items[item] = monkeys[monkey].items[item] % lcma;

                if monkeys[monkey].items[item] % monkeys[monkey].test as u128 == 0 {
                    let moving_item = monkeys[monkey].items.remove(item);
                    let monkey_true = monkeys[monkey].throw_if_true;

                    monkeys[monkey_true as usize].items.push(moving_item);
                } else {
                    let moving_item = monkeys[monkey].items.remove(item);
                    let monkey_false = monkeys[monkey].throw_if_false;

                    monkeys[monkey_false as usize].items.push(moving_item);
                }
            }
            
        }
    }

    let mut results = vec![];

    for monkey in monkeys {
        results.push(monkey.inspections);
    }

    results.sort();

    println!("{:?}", results[results.len() - 2] as u128 * results[results.len() - 1] as u128);
}