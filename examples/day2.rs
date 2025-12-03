use std::fs;

fn main() {
    const FILE_PATH: &str = "input/day2.txt";
    let contents = fs::read_to_string(FILE_PATH).unwrap();

    let res: u64 = contents
        .split(",")
        .flat_map(|r| {
            let (lower, upper) = r.split_once("-").unwrap();
            let lower = lower.parse().unwrap();
            let upper = upper.parse().unwrap();

            (lower..=upper).filter(predicate)
        })
        .sum();

    println!("Result: {res}");
}

fn predicate(num: &u64) -> bool {
    let digits: Box<_> = num.to_string().bytes().map(|b| b - b'0').collect();

    'outer: for i in 1..=digits.len() / 2 {
        if digits.len() % i != 0 {
            continue;
        }

        let pat: Box<_> = digits.iter().take(i).cloned().collect();

        for i in pat.len()..digits.len() {
            if digits[i] != pat[i % pat.len()] {
                continue 'outer;
            }
        }

        return true;
    }

    false
}
