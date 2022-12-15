use crate::util::read_lines;
use std::collections::HashSet;

struct Grid {
    grid: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Grid {
    fn parse_input() -> Grid {
        let lines = read_lines("input/d3").filter_map(|r| r.ok());
        let mut grid = Vec::new();
        let mut line_len = 0;
        for line in lines {
            grid.push(line.as_str().chars().collect::<Vec<char>>());
            line_len = line.len();
        }
        Grid {
            height: grid.len(),
            grid,
            width: line_len,
        }
    }

    fn rating(&self, ox: bool) -> usize {
        let mut rows = HashSet::from_iter(0..self.height);
        for i in 0..self.width {
            rows = self.calc(i, ox, rows);
            if rows.len() == 1 {
                break;
            }
        }
        let index = rows.iter().next().unwrap();
        let s = self
            .grid
            .get(*index)
            .unwrap()
            .iter()
            .collect::<String>();
        usize::from_str_radix(s.as_str(), 2).unwrap()
    }

    fn calc(&self, col: usize, most_common: bool, included: HashSet<usize>) -> HashSet<usize> {
        let ones = self
            .grid
            .iter()
            .enumerate()
            .filter(|(i, _)| included.contains(i))
            .map(|(_, row)| row.get(col).unwrap())
            .fold(0, |accu, e| {
                if *e == '1' {
                    return accu + 1;
                }
                return accu;
            });

        if ones + ones < included.len() {
            return self.take(if most_common { '0' } else { '1' }, col, included);
        }
        return self.take(if most_common { '1' } else { '0' }, col, included);
    }

    fn take(&self, c: char, col: usize, included: HashSet<usize>) -> HashSet<usize> {
        self.grid
            .iter()
            .enumerate()
            .filter(|(i, _)| included.contains(i))
            .filter(|(_, row)| *row.get(col).unwrap() == c)
            .map(|(i, _)| i)
            .collect::<HashSet<usize>>()
    }
}

#[allow(dead_code)]
fn p1() {
    let mut count = 0;
    let mut bit_counts = Vec::new();
    for line in read_lines("input/d3") {
        let s = line.unwrap();
        while s.len() > bit_counts.len() {
            bit_counts.push(0);
        }
        for (i, c) in s.as_str().chars().enumerate() {
            if let '1' = c {
                bit_counts[i] += 1
            }
        }
        count += 1;
    }

    let mut gamma = String::new();
    let mut epsilon = String::new();
    for bit in bit_counts {
        if bit + bit < count {
            gamma.push('0');
            epsilon.push('1');
        } else {
            gamma.push('1');
            epsilon.push('0');
        }
    }
    println!(
        "{}",
        usize::from_str_radix(gamma.as_str(), 2).unwrap()
            * usize::from_str_radix(epsilon.as_str(), 2).unwrap()
    );
}

fn p2() {
    let grid = Grid::parse_input();
    dbg!(grid.rating(true) * grid.rating(false));
}

pub fn main() {
    p2();
}
