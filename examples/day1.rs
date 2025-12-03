use std::fs;

fn main() {
    const FILE_PATH: &str = "input/day1.txt";
    let contents = fs::read_to_string(FILE_PATH).unwrap();

    let mut zeros = 0;
    let mut rot = 50;
    for line in contents.split("\r\n") {
        let diff: i32 = line.replace("L", "-").replace("R", "").parse().unwrap();
        rot = (rot + 100 + diff) % 100;

        if rot == 0 {
            zeros += 1
        }
    }

    println!("password: {zeros}")
}
