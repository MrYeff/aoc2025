use std::{
    convert::Infallible,
    fs,
    ops::{Index, IndexMut},
    str::FromStr,
};

const OFFSETS: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn main() {
    const FILE_PATH: &str = "input/day4.txt";
    let contents = fs::read_to_string(FILE_PATH).unwrap();

    let grid = grid_from_text(contents, |c| c == '@');

    let mut sum = 0;
    for y in 0..grid.height {
        for x in 0..grid.width {
            if !grid[(x, y)] {
                continue;
            }

            let count: u8 = OFFSETS
                .iter()
                .filter_map(|(dx, dy)| {
                    let x = (x as isize + dx).try_into().ok()?;
                    let y = (y as isize + dy).try_into().ok()?;

                    let val = grid.get((x, y))?;
                    val.then_some(1)
                })
                .sum();

            if count < 4 {
                sum += 1;
            }
        }
    }

    println!("reachable: {sum}")
}

fn grid_from_text<T: Default + Clone>(text: String, f: impl Fn(char) -> T) -> Grid<T> {
    let w = text.lines().next().unwrap().len();
    let h = text.lines().count();
    let mut out = Grid::new(w, h);

    for (y, line) in text.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            out[(x, y)] = f(char)
        }
    }

    out
}

fn grid_to_text<T: Default + Clone>(grid: Grid<T>, f: impl Fn(&T) -> char) -> String {
    let mut text = String::with_capacity(grid.width * grid.height);

    for y in 0..grid.height {
        for x in 0..grid.width {
            text.push(f(&grid[(x, y)]));
        }
        text.push_str("\r\n");
    }

    text
}

struct Grid<T> {
    width: usize,
    height: usize,
    data: Box<[T]>,
}

impl<T> Grid<T> {
    fn cell(&self, (x, y): (usize, usize)) -> usize {
        self.width * y + x
    }

    fn new(width: usize, height: usize) -> Self
    where
        T: Default + Clone,
    {
        Self {
            width,
            height,
            data: vec![T::default(); width * height].into_boxed_slice(),
        }
    }

    fn get(&self, (x, y): (usize, usize)) -> Option<&T> {
        (x < self.width && y < self.height).then(|| &self[(x, y)])
    }

    fn get_mut(&mut self, (x, y): (usize, usize)) -> Option<&mut T> {
        (x < self.width && y < self.height).then(|| &mut self[(x, y)])
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[self.cell(index)]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[self.cell(index)]
    }
}

#[cfg(test)]
mod tests {
    pub use super::*;
    use test_case::test_case;
}
