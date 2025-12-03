use std::fs;

#[derive(Debug, Eq, PartialEq)]
struct Range {
    lower: u64,
    upper: u64,
}

fn main() {
    const FILE_PATH: &str = "input/day2.txt";
    let contents = fs::read_to_string(FILE_PATH).unwrap();

    let ranges = contents.split(",").map(|r| {
        let (lower, upper) = r.split_once("-").unwrap();
        Range {
            lower: lower.parse().unwrap(),
            upper: upper.parse().unwrap(),
        }
    });

    let res = process(ranges);
    println!("Result: {res}");
}

///12 - 88 -> 12 - 88
///12 - 123 -> 12 - 99 | 100 - 123
///12 - 1234 -> 12 - 99 | 100 - 999 | 1000 - 1234
fn split_ranges(ranges: impl Iterator<Item = Range>) -> impl Iterator<Item = Range> {
    let mut out = Vec::new();
    for range in ranges {
        let lower_len = range.lower.ilog10() + 1;
        let upper_len = range.upper.ilog10() + 1;
        if lower_len == upper_len {
            out.push(range);
            continue;
        }

        let mut current_lower = range.lower;
        for len in lower_len..upper_len {
            let mag = 10u64.pow(len);
            out.push(Range {
                lower: current_lower,
                upper: mag - 1,
            });
            current_lower = mag;
        }
        out.push(Range {
            lower: current_lower,
            upper: range.upper,
        });
    }
    out.into_iter()
}

fn process(ranges: impl Iterator<Item = Range>) -> u64 {
    let ranges = split_ranges(ranges);
    let mut sum = 0;

    for range in ranges {
        let len = range.lower.ilog10() + 1;
        if len % 2 == 1 {
            continue;
        }
        let (l, r) = split(range.lower);
        let start = l + (l < r) as u64;
        // 456|123 -> 456|456
        // 123|456 -> 124|124

        let (l, r) = split(range.upper);
        let end = l - (l > r) as u64;
        // 123|456 -> 123|123
        // 456|123 -> 455|455

        for i in start..=end {
            let mag = 10u64.pow(i.ilog10() + 1);
            let val = i * mag + i;
            sum += val;
        }
    }

    sum
}

fn split(x: u64) -> (u64, u64) {
    let halflen = x.ilog10().div_ceil(2);
    let mag = 10u64.pow(halflen);
    (x / mag, x % mag)
}

#[cfg(test)]
mod tests {
    pub use super::*;
    use test_case::test_case;

    impl Range {
        fn new(lower: u64, upper: u64) -> Self {
            Self { lower, upper }
        }
    }

    #[test_case(vec![Range::new(12,88)], vec![Range::new(12,88)]; "1 mag")]
    #[test_case(vec![Range::new(12,123)], vec![Range::new(12,99),Range::new(100,123)]; "2 mag")]
    #[test_case(vec![Range::new(12,1234)], vec![Range::new(12,99),Range::new(100,999),Range::new(1000,1234)]; "3 mag")]
    fn split_ranges_test(xs: Vec<Range>, ys: Vec<Range>) {
        let rs: Vec<_> = split_ranges(xs.into_iter()).collect();
        assert_eq!(rs, ys);
    }
}
