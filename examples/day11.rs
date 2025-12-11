use std::{collections::HashMap, fs, iter::once};
fn main() {
    const FILE_PATH: &str = "input/day11.txt";
    let contents = fs::read_to_string(FILE_PATH).unwrap();
    let dependents: HashMap<Sid, Vec<Sid>> = contents.lines().map(extract_server).collect();

    const START: Sid = Sid("svr");
    const GOAL: Sid = Sid("out");
    const HOP_A: Sid = Sid("dac");
    const HOP_B: Sid = Sid("fft");

    let mut mem: HashMap<Sid, u64> = once((GOAL, 1)).collect();
    let hop_a1 = solve(&mut mem, &dependents, HOP_A);
    let hop_b1 = solve(&mut mem, &dependents, HOP_B);

    let mut mem: HashMap<Sid, u64> = once((HOP_A, hop_a1)).collect();
    let hop_b2 = solve(&mut mem, &dependents, HOP_B);

    let mut mem: HashMap<Sid, u64> = once((HOP_B, hop_b1)).collect();
    let hop_a2 = solve(&mut mem, &dependents, HOP_A);

    let mut mem: HashMap<Sid, u64> = [(HOP_A, hop_a2), (HOP_B, hop_b2)].into_iter().collect();
    let result = solve(&mut mem, &dependents, START);

    println!("result {result}")
}

fn solve<'a>(
    mem: &mut HashMap<Sid<'a>, u64>,
    dependents: &'a HashMap<Sid, Vec<Sid>>,
    x: Sid<'a>,
) -> u64 {
    if let Some(y) = mem.get(&x) {
        return *y;
    }

    let Some(deps) = dependents.get(&x) else {
        return 0;
    };

    let y = deps.iter().map(|x| solve(mem, dependents, *x)).sum();
    mem.insert(x, y);
    y
}

fn extract_server(l: &str) -> (Sid, Vec<Sid>) {
    let (k, v) = l.split_once(": ").unwrap();
    (Sid(k), v.split_whitespace().map(Sid).collect())
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Sid<'a>(&'a str);
