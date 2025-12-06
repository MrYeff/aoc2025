use std::fs;

fn main() {
    const FILE_PATH: &str = "input/day5.txt";
    let contents = fs::read_to_string(FILE_PATH).unwrap();
    let mut lines = contents.lines();

    let mut ranges: Vec<(u64, u64)> = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (l, u) = line.split_once('-').unwrap();
            (l.parse().unwrap(), u.parse().unwrap())
        })
        .collect();

    ranges.sort_by_key(|r| r.0);

    let mut count = 0;
    let mut prev_u = 0;
    for range in ranges.into_iter() {
        if prev_u >= range.1 {
            // completly inside prev
            continue;
        }

        count += range.1 - range.0;

        if prev_u >= range.0 {
            // overlapping ranges
            count -= prev_u - range.0
        } else {
            // count each element not length only 3-5 = 3 (3,4,5)
            count += 1
        }

        prev_u = range.1;
    }
    println!("count {count}")
}

#[cfg(test)]
mod tests {
    pub use super::*;
    use test_case::test_case;
}
