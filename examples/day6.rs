use std::fs;

enum Op {
    Add,
    Mul,
}

fn main() {
    const FILE_PATH: &str = "input/day6.txt";
    let contents = fs::read_to_string(FILE_PATH).unwrap();
    let mut lines = contents.lines();
    let ops: Box<_> = lines
        .next_back()
        .unwrap()
        .split_whitespace()
        .map(|c| match c.chars().next().unwrap() {
            '+' => Op::Add,
            '*' => Op::Mul,
            x => panic!("unexpected opperand: '{x}'"),
        })
        .collect();

    let mut out: Box<[u64]> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    for line in lines {
        for (i, x) in line
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .enumerate()
        {
            match ops[i] {
                Op::Add => out[i] += x,
                Op::Mul => out[i] *= x,
            }
        }
    }

    let checksum: u64 = out.into_iter().sum();
    println!("checksum {checksum}");
}

#[cfg(test)]
mod tests {
    pub use super::*;
    use test_case::test_case;
}
