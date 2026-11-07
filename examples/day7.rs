use edition2025::U256;
use std::fs;

fn main() {
    const FILE_PATH: &str = "input/day7.txt";
    let contents = fs::read_to_string(FILE_PATH).unwrap();

    // Parsing

    let mut lines = contents.lines().step_by(2);
    let start: usize = lines.by_ref().next().unwrap().find('S').unwrap();

    let splitters = lines.map(|l: &str| {
        l.match_indices('^')
            .map(|(i, _)| i)
            .fold(U256::ZERO, |acc, x| acc | U256::ONE << x)
    });

    // Solve

    let mut beams = U256::ONE << start;
    let splits: usize = splitters
        .map(move |mask| {
            let passes = beams & !mask;
            let hits = beams & mask;

            beams = hits >> 1 | hits << 1 | passes;

            hits.count_ones()
        })
        .sum();

    println!("Total splits: {}", splits);
}
