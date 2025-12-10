use std::{collections::HashSet, fmt::Debug, fs};

//[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}

fn main() {
    const FILE_PATH: &str = "input/day10.txt";
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

    let mut d = 1;
    let mut history = HashSet::new();
    let mut states = vec![m.target.clone()];
    loop {
        for state in states
            .drain(0..states.len())
            .collect::<Box<_>>()
            .into_iter()
        {
            for butt in m.buttons.iter() {
                if let Some(state) = apply_button(&state, &butt) {
                    if check_0(&state) {
                        return d;
                    }

                    if !history.contains(&state) {
                        states.push(state);
                    }
                }
            }

            history.insert(state);
        }

        states.sort();
        states.dedup();

        d += 1;

        println!("Histroy {}", history.len())
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
