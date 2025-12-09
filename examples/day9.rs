use itertools::*;
use std::fs;

fn main() {
    const FILE_PATH: &str = "input/day9.txt";
    let contents = fs::read_to_string(FILE_PATH).unwrap();

    let max_area = contents
        .lines()
        .map(|l| l.split_once(',').unwrap())
        .map(|s| (s.0.parse::<u64>().unwrap(), s.1.parse::<u64>().unwrap()))
        .tuple_combinations()
        .map(|(a, b)| (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1))
        .max()
        .unwrap();

    println!("max_area: {max_area}")
}

#[cfg(test)]
mod tests {
    pub use super::*;
    use test_case::test_case;
}
