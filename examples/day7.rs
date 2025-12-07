use std::{fs, iter::repeat, mem};

fn main() {
    const FILE_PATH: &str = "input/day7.txt";
    let contents = fs::read_to_string(FILE_PATH).unwrap();

    let mut lines = contents.lines();

    let mut prev_state: Box<_> = lines
        .next()
        .unwrap()
        .as_bytes()
        .iter()
        .map(|b| (*b == b'S') as u64)
        .collect();

    let mut state: Box<_> = repeat(0).take(prev_state.len()).collect();

    for line in lines {
        for (i, (ps, b)) in prev_state.iter().zip(line.bytes()).enumerate() {
            if *ps == 0 {
                continue;
            }
            match b {
                b'.' => state[i] += ps,
                b'^' => {
                    state[i - 1] += ps;
                    state[i + 1] += ps;
                }
                _ => {}
            }
        }
        mem::swap(&mut prev_state, &mut state);
        state.iter_mut().for_each(|s| *s = 0);
    }

    let world_count: u64 = prev_state.into_iter().sum();
    println!("worlds {world_count}")
}

#[cfg(test)]
mod tests {
    pub use super::*;
    use test_case::test_case;
}
