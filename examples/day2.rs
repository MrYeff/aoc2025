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
    let (l, r) = split(*num);
    l == r
}

fn split(x: u64) -> (u64, u64) {
    let halflen = x.ilog10().div_ceil(2);
    let mag = 10u64.pow(halflen);
    (x / mag, x % mag)
}
