use std::collections::HashSet;

use crate::util::read_lines;

#[derive(Debug)]
struct Bingo {
    board: Vec<usize>,
    unmarked: HashSet<usize>,
    won: bool,
}

#[derive(Debug)]
enum MarkResult {
    Win(usize),
    NoMatch,
    NotWin,
    Won,
}

const WIN_INDICES: [[usize; 5]; 12] = [
    [0, 1, 2, 3, 4],
    [5, 6, 7, 8, 9],
    [10, 11, 12, 13, 14],
    [15, 16, 17, 18, 19],
    [20, 21, 22, 23, 24],
    [0, 5, 10, 15, 20],
    [1, 6, 11, 16, 21],
    [2, 7, 12, 17, 22],
    [3, 8, 13, 18, 23],
    [4, 9, 14, 19, 24],
    [0, 6, 12, 18, 24],
    [4, 8, 12, 16, 20],
];

impl Bingo {
    fn new(board: Vec<usize>) -> Self {
        Self {
            unmarked: HashSet::from_iter(board.iter().cloned()),
            board,
            won: false,
        }
    }

    fn mark(&mut self, n: usize) -> MarkResult {
        if self.is_won() {
            return MarkResult::Won;
        }
        if self.unmarked.contains(&n) {
            self.unmarked.remove(&n);
            if self.is_won() {
                return MarkResult::Win(n * self.unmarked.iter().fold(0, |accum, e| accum + *e));
            } else {
                return MarkResult::NotWin;
            }
        }
        MarkResult::NoMatch
    }

    fn is_won(&mut self) -> bool {
        if self.won {
            return true;
        }

        for range in WIN_INDICES {
            let mut win = true;
            for i in range {
                if self.unmarked.contains(self.board.get(i).unwrap()) {
                    win = false;
                    break;
                }
            }
            if win {
                self.won = true;
                return true;
            }
        }
        return false;
    }
}

#[allow(dead_code)]
fn p1() {
    let mut lines = read_lines("input/d4");
    let nums = lines
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|s| usize::from_str_radix(s, 10).unwrap())
        .collect::<Vec<usize>>();

    let mut bingos = Vec::new();
    let mut bingo = Vec::new();
    for line in lines {
        let line = line.unwrap();
        if line.trim().is_empty() {
            continue;
        }

        for s in line.split(' ') {
            if s.is_empty() {
                continue;
            }
            bingo.push(usize::from_str_radix(s, 10).unwrap())
        }
        if bingo.len() == 25 {
            bingos.push(Bingo::new(bingo));
            bingo = Vec::new();
        }
    }

    let mut win_scores = Vec::new();
    for n in nums {
        for b in bingos.iter_mut() {
            match b.mark(n) {
                MarkResult::Win(s) => {
                    win_scores.push(s);
                }
                MarkResult::NoMatch => (),
                MarkResult::NotWin => (),
                MarkResult::Won => (),
            }
        }
    }
    dbg!(win_scores);
}

#[allow(dead_code)]
pub fn main() {
    p1();
}
