use std::fs;

fn main() {
    const FILE_PATH: &str = "input/day3.txt";
    let contents = fs::read_to_string(FILE_PATH).unwrap();

    let joltage: u32 = contents
        .split("\r\n")
        .map(|line| line.bytes().map(|b| b - b'0'))
        .map(extract_joltage)
        .sum();

    println!("joltage: {joltage}")
}

fn extract_joltage(seq: impl Iterator<Item = u8>) -> u32 {
    let seq: Box<_> = seq.collect();

    let mut l = seq[0];
    let mut r = seq[1];

    for i in 2..seq.len() {
        let val = seq[i];

        if r > l {
            l = r;
            r = val;
            continue;
        }

        if val > r {
            r = val;
        }
    }

    (l * 10 + r) as u32
}

#[cfg(test)]
mod tests {
    pub use super::*;
    use test_case::test_case;

    #[test_case("987654321111111", 98)]
    #[test_case("811111111111119", 89)]
    #[test_case("234234234234278", 78)]
    #[test_case("818181911112111", 92)]
    #[test_case("892", 92)]
    #[test_case("1234", 34)]
    #[test_case("9811", 98)]
    #[test_case("9217", 97)]
    fn test_extract_joltage(seq: &'static str, expected: u32) {
        let got = extract_joltage(seq.bytes().map(|b| b - b'0'));
        assert_eq!(got, expected);
    }
}
