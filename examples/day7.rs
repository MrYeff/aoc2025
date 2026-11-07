use std::{collections::HashSet, fs};

fn main() {
    const FILE_PATH: &str = "input/day7.txt";
    const LINE_LEN: usize = 142;

    let contents = fs::read_to_string(FILE_PATH).unwrap();

    // Parsing

    let mut lines = contents.lines().step_by(2);
    let start: usize = lines.by_ref().next().unwrap().find('S').unwrap();

    let splitters =
        lines.map(|l: &str| l.match_indices('^').map(|(i, _)| i).collect::<HashSet<_>>());

    // Solve

    let mut beams = vec![false; LINE_LEN];
    beams[start] = true;

    let splits: usize = splitters
        .map(|mask| {
            let mut beams_next = vec![false; LINE_LEN];

            let mut splits = 0;
            (0..LINE_LEN).for_each(|x| {
                if beams[x] == false {
                    return;
                }

                if mask.contains(&x) {
                    beams_next[x - 1] |= beams[x];
                    beams_next[x + 1] |= beams[x];
                    splits += 1;
                } else {
                    beams_next[x] |= beams[x];
                }
            });

            beams = beams_next;
            splits
        })
        .sum();

    println!("Total splits: {}", splits);
}
