use std::{collections::HashMap, fs, iter::once};
fn main() {
    const FILE_PATH: &str = "input/day11.txt";
    let contents = fs::read_to_string(FILE_PATH).unwrap();
    let dependents: HashMap<Sid, Vec<Sid>> = contents.lines().map(extract_server).collect();

    const START: Sid = Sid("you");
    const GOAL: Sid = Sid("out");

    let mut mem: HashMap<Sid, u32> = once((GOAL, 1)).collect();
    let result = solve(&mut mem, &dependents, START);

    println!("result {result}")
}

fn solve<'a>(
    mem: &mut HashMap<Sid<'a>, u32>,
    dependents: &'a HashMap<Sid, Vec<Sid>>,
    x: Sid<'a>,
) -> u32 {
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
