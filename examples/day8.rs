use std::{
    collections::{HashMap, HashSet},
    fs,
};

use itertools::Itertools;

type CBox = (u64, u64, u64);

fn main() {
    const FILE_PATH: &str = "input/day8.txt";
    let contents = fs::read_to_string(FILE_PATH).unwrap();

    let boxes: Vec<CBox> = contents.lines().map(extract_box).collect();

    let mut pairs: Vec<(CBox, CBox)> = boxes.iter().cloned().tuple_combinations().collect();
    pairs.sort_by_key(|(a, b)| dist_mag(a, b));

    let mut grid = Grid::new();
    let (last_a, last_b) = pairs
        .iter()
        .filter(|(a, b)| grid.connect_boxes(a, b))
        .last()
        .unwrap();

    let result = last_a.0 * last_b.0;
    println!("result: {result}")
}

fn extract_box(l: &str) -> CBox {
    let mut s = l.split(',').map(|n| n.parse().unwrap());
    (s.next().unwrap(), s.next().unwrap(), s.next().unwrap())
}

fn dist_mag((x1, y1, z1): &CBox, (x2, y2, z2): &CBox) -> u64 {
    let dx = x1.abs_diff(*x2);
    let dy = y1.abs_diff(*y2);
    let dz = z1.abs_diff(*z2);

    dx * dx + dy * dy + dz * dz
}

struct Grid {
    circuits: Vec<Option<HashSet<CBox>>>,
    circuit_refs: HashMap<CBox, usize>,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            circuits: Vec::new(),
            circuit_refs: HashMap::new(),
        }
    }

    /// connect the 2 boxes and return if a and b where not previously unconnected
    pub fn connect_boxes(&mut self, a: &CBox, b: &CBox) -> bool {
        match (
            self.circuit_refs.get(a).copied(),
            self.circuit_refs.get(b).copied(),
        ) {
            (Some(ac), None) => self.insert_into_circuit(ac, *b),
            (None, Some(bc)) => self.insert_into_circuit(bc, *a),
            (None, None) => self.new_circuit(*a, *b),
            (Some(ac), Some(bc)) if ac == bc => return false,
            (Some(ac), Some(bc)) => self.overwrite(ac, bc),
        }
        true
    }

    fn insert_into_circuit(&mut self, c: usize, x: CBox) {
        self.circuits[c].as_mut().unwrap().insert(x);
        self.circuit_refs.insert(x, c);
    }

    fn new_circuit(&mut self, a: CBox, b: CBox) {
        self.circuits.push(Some([a, b].into_iter().collect()));
        let idx = self.circuits.len() - 1;
        self.circuit_refs.insert(a, idx);
        self.circuit_refs.insert(b, idx);
    }

    fn overwrite(&mut self, new: usize, prev: usize) {
        let prev_boxes_b = self.circuits[prev].take().unwrap();

        for bcp in prev_boxes_b {
            *self.circuit_refs.get_mut(&bcp).unwrap() = new;
            self.circuits[new].as_mut().unwrap().insert(bcp);
        }
    }
}
