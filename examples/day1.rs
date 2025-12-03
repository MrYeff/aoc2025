use std::fs;

fn main() {
    const FILE_PATH: &str = "input/day1.txt";
    let contents = fs::read_to_string(FILE_PATH).unwrap();

    let seq = contents
        .split("\r\n")
        .map(|line| line.replace("L", "-").replace("R", "").parse().unwrap());
    let zeros = process(50, seq);

    println!("password: {zeros}")
}

fn process(start: i32, seq: impl Iterator<Item = i32>) -> u32 {
    let mut zeros: u32 = 0;
    let mut rot = start;
    for diff in seq {
        zeros += i32::abs(diff / 100) as u32;

        let diff = diff % 100;
        if diff == 0 {
            continue;
        }

        let start_rot = rot;
        rot += diff;

        match rot {
            0 => {
                zeros += 1;
            }
            r if r >= 100 => {
                rot -= 100;
                zeros += 1;
            }
            r if r < 0 => {
                rot += 100;
                if start_rot != 0 {
                    zeros += 1;
                }
            }
            _ => {}
        }
    }
    zeros
}

#[cfg(test)]
mod tests {
    pub use super::*;
    use test_case::test_case;

    #[test_case(0,  vec![0],    0; "0R0")]
    #[test_case(0,  vec![100],  1; "0R100")]
    #[test_case(0,  vec![-100], 1; "0L100")]
    #[test_case(0,  vec![99],   0; "0R99")]
    #[test_case(0,  vec![999],  9; "0R999")]
    #[test_case(50, vec![100],  1; "50R100")]
    #[test_case(50, vec![-100], 1; "50L100")]
    #[test_case(50, vec![-50],  1; "50L50")]
    #[test_case(50, vec![50],   1; "50R50")]
    #[test_case(50, vec![1000], 10; "50R1000")]
    #[test_case(50, vec![950],  10; "50R950")]
    #[test_case(50, vec![-950],  10; "50L950")]
    #[test_case(0, vec![100, 0], 1; "0R100,0")]
    #[test_case(0, vec![100, 100], 2; "0R100,100")]
    #[test_case(0, vec![-150], 1; "0L150")]
    fn zeros_cases(start: i32, moves: Vec<i32>, expected: u32) {
        let got = process(start, moves.into_iter());
        assert_eq!(got, expected);
    }
}
