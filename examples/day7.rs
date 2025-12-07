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
        .map(|b| *b == b'S')
        .collect();

    let mut state: Box<_> = repeat(false).take(prev_state.len()).collect();
    let mut splits = 0;

    for line in lines {
        for (i, (ps, b)) in prev_state.iter().zip(line.bytes()).enumerate() {
            if !ps {
                continue;
            }
            match b {
                b'.' => state[i] = true,
                b'^' => {
                    splits += 1;
                    state[i - 1] = true;
                    state[i + 1] = true;
                }
                _ => {}
            }
        }
        mem::swap(&mut prev_state, &mut state);
        state.iter_mut().for_each(|s| *s = false);
    }

    println!("splits {splits}")
}

#[cfg(test)]
mod tests {
    pub use super::*;
    use test_case::test_case;
}
