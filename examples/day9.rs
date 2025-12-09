use itertools::*;
use std::fs;

fn main() {
    const FILE_PATH: &str = "input/day9.txt";
    let contents = fs::read_to_string(FILE_PATH).unwrap();
    let points: Box<[(u64, u64)]> = contents
        .lines()
        .map(|l| l.split_once(',').unwrap())
        .map(|s| (s.0.parse().unwrap(), s.1.parse().unwrap()))
        .collect();

    let neighbours: Box<_> = points
        .iter()
        .zip(points.iter().cycle().skip(1))
        .map(|(a, b)| (*a, *b))
        .collect();

    let max_area = points
        .iter()
        .cloned()
        .tuple_combinations()
        .filter(|outer| {
            !neighbours
                .iter()
                .any(|inner| segment_inside_rect(*inner, *outer))
        })
        .map(|(a, b)| (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1))
        .max()
        .unwrap();

    println!("max_area: {max_area}");
}

fn segment_inside_rect(s: ((u64, u64), (u64, u64)), r: ((u64, u64), (u64, u64))) -> bool {
    let (sx0, sx1) = min_max(s.0.0, s.1.0);
    let (sy0, sy1) = min_max(s.0.1, s.1.1);
    let (rx0, rx1) = min_max(r.0.0, r.1.0);
    let (ry0, ry1) = min_max(r.0.1, r.1.1);

    if sy0 == sy1 {
        let sy = sy0;
        (ry0 < sy && sy < ry1) && (sx0 < rx1 && rx0 < sx1)
    } else {
        let sx = sx0;
        (rx0 < sx && sx < rx1) && (sy0 < ry1 && ry0 < sy1)
    }
}

fn min_max(a: u64, b: u64) -> (u64, u64) {
    if a > b { (b, a) } else { (a, b) }
}

#[cfg(test)]
mod tests {
    pub use super::*;
    use test_case::test_case;
}
