use itertools::Itertools;
use std::{fmt::Debug, fs, iter::repeat};

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
        .map(|x| (x == b'#') as u32)
        .collect();

    let splt = splt.rev().skip(1);

    let buttons = splt.map(extract_button).collect();

    Machine { target, buttons }
}

fn extract_button(s: &str) -> Box<[usize]> {
    s[1..s.len() - 1]
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

fn search(m: Machine) -> usize {
    let state_0: Box<[u32]> = repeat(0).take(m.target.len()).collect();

    if m.target.iter().all(|x| *x == 0) {
        return 0;
    }

    for d in 1..=m.buttons.len() {
        if m.buttons
            .iter()
            .combinations(d)
            .map(|seq| {
                let mut acc = state_0.clone();
                seq.iter().for_each(|x| apply_button(acc.as_mut(), x));
                acc
            })
            .any(|s| validate_target(&m.target, &s))
        {
            return d;
        }
    }

    unreachable!("unsolvable")
}

fn validate_target(ts: &[u32], xs: &[u32]) -> bool {
    ts.iter().zip(xs.iter()).all(|(t, x)| *t == *x)
}

fn apply_button(ss: &mut [u32], xs: &[usize]) {
    xs.iter().for_each(|x| ss[*x] = (ss[*x] + 1) % 2);
}

#[derive(Debug)]
struct Machine {
    target: Box<[u32]>,
    buttons: Vec<Box<[usize]>>,
}
