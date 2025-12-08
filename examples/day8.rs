use std::{
    collections::{HashMap, HashSet},
    fs,
    iter::repeat,
};

fn main() {
    const FILE_PATH: &str = "input/day8.txt";
    const CONNECTION_COUNT: usize = 1000;
    let contents = fs::read_to_string(FILE_PATH).unwrap();

    let points: Box<[(u64, u64, u64)]> = contents
        .lines()
        .map(|l| {
            let mut s = l.split(',').map(|n| n.parse().unwrap());
            (s.next().unwrap(), s.next().unwrap(), s.next().unwrap())
        })
        .collect();

    let mut pairs: Vec<_> = points
        .iter()
        .enumerate()
        .flat_map(|(i, &a)| points[i + 1..].iter().map(move |&b| (a, b)))
        .collect();

    pairs.sort_by_key(|(a, b)| dist_mag(a, b));

    let mut circuits: Vec<Option<HashSet<(u64, u64, u64)>>> = Vec::new();
    let mut circuit_refs: HashMap<(u64, u64, u64), usize> = HashMap::new();

    for (a, b) in pairs.iter().take(CONNECTION_COUNT) {
        //
        match (circuit_refs.contains_key(a), circuit_refs.contains_key(b)) {
            (true, false) => {
                let ac = circuit_refs.get(a).unwrap();
                circuits[*ac].as_mut().unwrap().insert(*b);
                circuit_refs.insert(*b, *ac);
            }
            (false, true) => {
                let bc = circuit_refs.get(b).unwrap();
                circuits[*bc].as_mut().unwrap().insert(*a);
                circuit_refs.insert(*a, *bc);
            }
            (false, false) => {
                circuits.push(Some([*a, *b].into_iter().collect()));
                circuit_refs.insert(*a, circuits.len() - 1);
                circuit_refs.insert(*b, circuits.len() - 1);
            }
            (true, true) => {
                let ac = *circuit_refs.get(a).unwrap();
                let bc = *circuit_refs.get(b).unwrap();

                if ac == bc {
                    continue;
                }

                for bcp in circuits[bc].take().unwrap() {
                    *circuit_refs.get_mut(&bcp).unwrap() = ac;
                    circuits[ac].as_mut().unwrap().insert(bcp);
                }
            }
        }
    }

    let mut lens: Vec<_> = circuits
        .into_iter()
        .filter_map(|v| v.map(|xs| xs.len()))
        .collect();

    lens.sort();
    let result = lens
        .into_iter()
        .rev()
        .chain(repeat(1))
        .take(3)
        .fold(1, |acc, x| acc * x);

    println!("result: {result}")
}

fn dist_mag((x1, y1, z1): &(u64, u64, u64), (x2, y2, z2): &(u64, u64, u64)) -> u64 {
    let dx = x1.abs_diff(*x2);
    let dy = y1.abs_diff(*y2);
    let dz = z1.abs_diff(*z2);

    dx * dx + dy * dy + dz * dz
}

#[cfg(test)]
mod tests {
    pub use super::*;
    use test_case::test_case;
}
