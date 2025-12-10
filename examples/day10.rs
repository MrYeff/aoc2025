use itertools::Itertools;
use std::{fmt::Debug, fs, iter::repeat, marker::PhantomData};

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
    let state_0: Box<[u32]> = repeat(0).take(m.target.len()).collect();

    if m.target.iter().all(|x| *x == 0) {
        return 0;
    }

    for d in 1..u32::MAX {
        if distributions(d, m.buttons.len())
            .map(|dst| {
                let mut acc = state_0.clone();
                m.buttons
                    .iter()
                    .zip(dst.into_iter())
                    .for_each(|(x, amo)| apply_button(acc.as_mut(), x, amo));
                acc
            })
            .any(|s| validate_target(&m.target, &s))
        {
            return d;
        }
    }

    unreachable!("unsolvable")
}

fn distributions(amo: u32, buckets: usize) -> Box<dyn Iterator<Item = Box<[u32]>>> {
    if amo == 0 {
        return Box::new(std::iter::once(repeat(0).take(buckets).collect()));
    };

    Box::new((0..buckets).flat_map(move |b| {
        let dsts = distributions(amo - 1, buckets);
        dsts.map(move |mut dst| {
            dst[b] += 1;
            dst
        })
    }))
}

fn validate_target(ts: &[u32], xs: &[u32]) -> bool {
    ts.iter().zip(xs.iter()).all(|(t, x)| *t == *x)
}

fn apply_button(ss: &mut [u32], xs: &[usize], amo: u32) {
    xs.iter().for_each(|x| ss[*x] += amo);
}

#[derive(Debug)]
struct Machine {
    target: Box<[u32]>,
    buttons: Vec<Box<[usize]>>,
}
