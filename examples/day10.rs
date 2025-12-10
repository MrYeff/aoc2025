use itertools::Itertools;
use std::{fmt::Debug, fs};

//[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}

fn main() {
    const FILE_PATH: &str = "input/day10.txt";
    let contents = fs::read_to_string(FILE_PATH).unwrap();

    let clicks: usize = contents.lines().map(extract_machine).map(search).sum();

    println!("clicks {clicks}")
}

fn extract_machine(s: &str) -> Machine {
    let mut splt = s.split_whitespace();
    let s = splt.next().unwrap();

    let target = s[1..s.len() - 1]
        .bytes()
        .rev()
        .fold(0, |acc, x| acc << 1 | (x == b'#') as u32);

    let splt = splt.rev().skip(1);

    let buttons = splt.map(extract_button).collect();

    Machine { target, buttons }
}

fn extract_button(s: &str) -> u32 {
    s[1..s.len() - 1]
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .fold(0, |acc, x| acc | 1 << x)
}

fn search(m: Machine) -> usize {
    if m.target == 0 {
        return 0;
    }

    for d in 1..=m.buttons.len() {
        if m.buttons
            .iter()
            .combinations(d)
            .map(|seq| seq.iter().fold(0, |acc, x| acc ^ **x))
            .any(|x| x == m.target)
        {
            return d;
        }
    }

    unreachable!("unsolvable")
}

struct Machine {
    target: u32,
    buttons: Vec<u32>,
}

impl Debug for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Machine")
            .field("target", &format_args!("{:b}", self.target))
            .field(
                "buttons",
                &self.buttons.iter().map(|x| format!("{:b}", x)).join(", "),
            )
            .finish()
    }
}
