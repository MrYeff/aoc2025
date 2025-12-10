use std::{collections::HashMap, fmt::Debug, fs, iter::repeat};

use itertools::{Either, Itertools};

//[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}

fn main() {
    const FILE_PATH: &str = "input/example/day10.txt";
    let contents = fs::read_to_string(FILE_PATH).unwrap();

    let clicks: u32 = contents.lines().map(extract_machine).map(search).sum();

    println!("clicks {clicks}")
}

fn extract_machine(s: &str) -> Machine {
    let splt = s.split_whitespace();
    let mut splt = splt.skip(1).collect::<Vec<_>>().into_iter().rev();
    let s = splt.next().unwrap();

    let target = s[1..s.len() - 1]
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let buttons = splt.map(extract_button).collect();

    Machine { target, buttons }
}

fn extract_button(s: &str) -> Box<[usize]> {
    s[1..s.len() - 1]
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

fn search(m: Machine) -> u32 {
    if check_0(&m.target) {
        return 0;
    }

    // let mut history: HashMap<Box<[u32]>, u32> = once((m.target.clone(), 0)).collect();
    let mut history: HashMap<Box<[u32]>, u32> = HashMap::with_capacity(100000);
    history.insert(m.target.clone(), 0);
    let goal: Box<[u32]> = repeat(0).take(m.target.len()).collect();

    down(&m, &mut history, &goal, &m.target, 1);

    *history.get(&goal).unwrap()
}

fn down(m: &Machine, history: &mut HashMap<Box<[u32]>, u32>, goal: &[u32], state: &[u32], d: u32) {
    if let Some(d_best) = history.get(goal) {
        if d >= *d_best {
            return;
        }
    }

    let (prevs, mut news): (Vec<_>, Vec<_>) = m
        .buttons
        .iter()
        .filter_map(|butt| apply_button(&state, &butt))
        .partition_map(|state| match history.get(&state) {
            Some(d) => Either::Left((state, *d)),
            None => Either::Right(state),
        });

    prevs
        .into_iter()
        .filter(|(_, d_prev)| *d_prev > d)
        .for_each(|(prev, _)| {
            history.insert(prev, d);
        });

    news.sort_by_key(|state| state.iter().sum::<u32>());
    for new in news.into_iter() {
        history.insert(new.clone(), d + 1);
        down(m, history, goal, &new, d + 1);
    }
}

fn check_0(state: &[u32]) -> bool {
    state.iter().all(|x| *x == 0)
}

fn apply_button(state: &[u32], butt: &[usize]) -> Option<Box<[u32]>> {
    let mut state: Box<[u32]> = Box::from(state);
    for x in butt {
        let Some(s) = state[*x].checked_sub(1) else {
            return None;
        };

        state[*x] = s
    }
    Some(state)
}

#[derive(Debug)]
struct Machine {
    target: Box<[u32]>,
    buttons: Vec<Box<[usize]>>,
}
