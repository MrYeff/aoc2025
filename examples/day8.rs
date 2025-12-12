use std::{
    collections::{HashMap, HashSet},
    fs,
};

type CBox = (u64, u64, u64);

fn main() {
    const FILE_PATH: &str = "input/day8.txt";
    let contents = fs::read_to_string(FILE_PATH).unwrap();

    let points: Box<[CBox]> = contents
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

    let mut circuits: Vec<Option<HashSet<CBox>>> = Vec::new();
    let mut circuit_refs: HashMap<CBox, usize> = HashMap::new();

    let (last_a, last_b) = pairs
        .iter()
        .filter(|(a, b)| connect_boxes(&mut circuits, &mut circuit_refs, a, b))
        .last()
        .unwrap();

    let result = last_a.0 * last_b.0;

    println!("result: {result}")
}

/// connect the 2 boxes and return if a and b where not previously unconnected
fn connect_boxes(
    circuits: &mut Vec<Option<HashSet<CBox>>>,
    circuit_refs: &mut HashMap<CBox, usize>,
    a: &CBox,
    b: &CBox,
) -> bool {
    match (circuit_refs.get(a).copied(), circuit_refs.get(b).copied()) {
        (Some(ac), None) => insert_into_circuit(circuits, circuit_refs, ac, *b),
        (None, Some(bc)) => insert_into_circuit(circuits, circuit_refs, bc, *a),
        (None, None) => new_circuit(circuits, circuit_refs, *a, *b),
        (Some(ac), Some(bc)) if ac == bc => return false,
        (Some(ac), Some(bc)) => overwrite(circuits, circuit_refs, ac, bc),
    }
    true
}

fn insert_into_circuit(
    circuits: &mut Vec<Option<HashSet<CBox>>>,
    circuit_refs: &mut HashMap<CBox, usize>,
    c: usize,
    x: CBox,
) {
    circuits[c].as_mut().unwrap().insert(x);
    circuit_refs.insert(x, c);
}

fn new_circuit(
    circuits: &mut Vec<Option<HashSet<CBox>>>,
    circuit_refs: &mut HashMap<CBox, usize>,
    a: CBox,
    b: CBox,
) {
    circuits.push(Some([a, b].into_iter().collect()));
    let idx = circuits.len() - 1;
    circuit_refs.insert(a, idx);
    circuit_refs.insert(b, idx);
}

fn overwrite(
    circuits: &mut Vec<Option<HashSet<CBox>>>,
    circuit_refs: &mut HashMap<CBox, usize>,
    new: usize,
    prev: usize,
) {
    let prev_boxes_b = circuits[prev].take().unwrap();

    for bcp in prev_boxes_b {
        *circuit_refs.get_mut(&bcp).unwrap() = new;
        circuits[new].as_mut().unwrap().insert(bcp);
    }
}

fn dist_mag((x1, y1, z1): &CBox, (x2, y2, z2): &CBox) -> u64 {
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
