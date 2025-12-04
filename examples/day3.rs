use std::fs;

fn main() {
    const FILE_PATH: &str = "input/day3.txt";
    let contents = fs::read_to_string(FILE_PATH).unwrap();

    let joltage: u64 = contents
        .lines()
        .map(|line| line.bytes().map(|b| b - b'0'))
        .map(extract_joltage)
        .sum();

    println!("joltage: {joltage}")
}

const STRIP_LEN: usize = 12;
fn extract_joltage(mut seq: impl Iterator<Item = u8>) -> u64 {
    let mut strip: Vec<_> = seq.by_ref().take(STRIP_LEN).collect();
    let seq: Box<_> = seq.collect();

    'outer: for i in 0..seq.len() {
        let val = seq[i];

        for i in 0..STRIP_LEN - 1 {
            if strip[i] < strip[i + 1] {
                strip.remove(i);
                strip.push(val);
                continue 'outer;
            }
        }

        let last = strip.last_mut().unwrap();
        if val > *last {
            *last = val;
        }
    }

    strip
        .into_iter()
        .rev()
        .map(|x| x as u64)
        .fold((0, 1), |(out, mul), x| (out + mul * x, mul * 10))
        .0
}

#[cfg(test)]
mod tests {
    pub use super::*;
    use test_case::test_case;

    #[test_case("987654321111111", 987654321111)]
    #[test_case("811111111111119", 811111111119)]
    #[test_case("234234234234278", 434234234278)]
    #[test_case("818181911112111", 888911112111)]
    fn test_extract_joltage(seq: &'static str, expected: u64) {
        let got = extract_joltage(seq.bytes().map(|b| b - b'0'));
        assert_eq!(got, expected);
    }
}
