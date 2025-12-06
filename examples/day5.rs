use std::fs;

fn main() {
    const FILE_PATH: &str = "input/day5.txt";
    let contents = fs::read_to_string(FILE_PATH).unwrap();
    let mut lines = contents.lines();
    let ranges: Vec<(u64, u64)> = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (l, u) = line.split_once('-').unwrap();
            (l.parse().unwrap(), u.parse().unwrap())
        })
        .collect();

    let mut fresh = 0;
    for id in lines {
        let id: u64 = id.parse().unwrap();
        if ranges.iter().any(|(l, u)| id >= *l && id <= *u) {
            fresh += 1;
        }
    }
    println!("Fresh IDs: {}", fresh);
}

#[cfg(test)]
mod tests {
    pub use super::*;
    use test_case::test_case;
}
