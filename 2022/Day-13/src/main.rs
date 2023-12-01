//! Day 13, Parts 1 & 2 of the advent of code 2022
//! 
//! I wasn't sure where to start with this so I used 
//! [https://fasterthanli.me/series/advent-of-code-2022/part-13](This Guide) 
//! By faster than lime

use serde::Deserialize;

use std::fmt;
fn main() {
    println!("Hello, world!");
}

// Using serde for easy deserialization given the JSON compatible
// Syntax
#[derive(Clone, Deserialize, Eq, PartialEq)]
#[serde(untagged)]
enum Node {
    Number(isize),
    List(Vec<Node>)
}

// Pretty Debug
impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{n}"),
            Self::List(n) => f.debug_list().entries(n).finish(),
        }
    }
}

impl Node {
    fn with_slice<T>(&self, f: impl FnOnce(&[Node]) -> T ) -> T {
        match self {
            Self::List(n) => f(&n[..]),
            Self::Number(n) => f(&[Self::Number(*n)])
        }
    }
}