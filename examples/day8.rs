use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    const FILE_PATH: &str = "input/day8.txt";
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
    let mut last_con_xs = (0, 0);

    for (a, b) in pairs.iter() {
        match (circuit_refs.contains_key(a), circuit_refs.contains_key(b)) {
            (true, false) => {
                let ac = circuit_refs.get(a).unwrap();
                circuits[*ac].as_mut().unwrap().insert(*b);
                circuit_refs.insert(*b, *ac);
                last_con_xs = (a.0, b.0);
            }
            (false, true) => {
                let bc = circuit_refs.get(b).unwrap();
                circuits[*bc].as_mut().unwrap().insert(*a);
                circuit_refs.insert(*a, *bc);
                last_con_xs = (a.0, b.0);
            }
            (false, false) => {
                circuits.push(Some([*a, *b].into_iter().collect()));
                circuit_refs.insert(*a, circuits.len() - 1);
                circuit_refs.insert(*b, circuits.len() - 1);
                last_con_xs = (a.0, b.0);
            }
            (true, true) => {
                let ac = *circuit_refs.get(a).unwrap();
                let bc = *circuit_refs.get(b).unwrap();

                if ac == bc {
                    continue;
                }
                last_con_xs = (a.0, b.0);

                for bcp in circuits[bc].take().unwrap() {
                    *circuit_refs.get_mut(&bcp).unwrap() = ac;
                    circuits[ac].as_mut().unwrap().insert(bcp);
                }
            }
        }
    }

    let result = last_con_xs.0 * last_con_xs.1;

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
