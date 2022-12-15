use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead, Lines};
use std::path::Path;

pub struct Window<T, I>
where
    T: Clone,
    I: Iterator<Item = T>,
{
    it: I,
    cur: VecDeque<T>,
    size: usize,
}

impl<T, I> Window<T, I>
where
    T: Clone,
    I: Iterator<Item = T>,
{
    fn owned_current_window(&self) -> Vec<T> {
        let mut res = Vec::new();
        for i in self.cur.iter() {
            res.push(i.clone());
        }
        res
    }
}

impl<T, I> Iterator for Window<T, I>
where
    T: Clone,
    I: Iterator<Item = T>,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.cur.len() < self.size {
            self.cur.push_back(self.it.next()?);
            if self.cur.len() == self.size {
                return Some(self.owned_current_window());
            }
        }
        self.cur.push_back(self.it.next()?);
        self.cur.pop_front();
        Some(self.owned_current_window())
    }
}

pub trait Windowing {
    type Item: Clone;
    type Iter: Iterator<Item = Self::Item>;

    fn window(self, size: usize) -> Window<Self::Item, Self::Iter>;
}

impl Windowing for Lines<io::BufReader<File>> {
    type Item = String;

    type Iter = impl Iterator<Item = String>;
    // type Iter = Map<Lines<BufReader<File>>, impl FnOnce(Result<String, io::Error>) -> String>;

    fn window(self, size: usize) -> Window<Self::Item, Self::Iter> {
        let w = Window {
            it: self.into_iter().filter_map(|r| r.ok()),
            cur: VecDeque::new(),
            size,
        };
        w
    }
}

pub fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("file cannot be opened");
    io::BufReader::new(file).lines()
}
