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

    let mut lines: Box<_> = lines.map(|line| line.chars()).collect();

    let mut checksum: u64 = 0;
    for op in ops.into_iter() {
        let mut y = None;
        'outer: loop {
            let mut x_str = String::with_capacity(lines.len());
            for line in lines.iter_mut() {
                let Some(c) = line.next() else {
                    break 'outer;
                };

                x_str.push(c);
            }

            let x_str = x_str.trim();
            if x_str.is_empty() {
                break;
            }

            let x: u64 = x_str.parse().unwrap();

            match y.as_mut() {
                Some(y) => match op {
                    Op::Add => *y += x,
                    Op::Mul => *y *= x,
                },
                None => y = Some(x),
            }
        }
        checksum += y.unwrap();
    }

    println!("checksum {checksum}");
}

#[cfg(test)]
mod tests {
    pub use super::*;
    use test_case::test_case;
}
